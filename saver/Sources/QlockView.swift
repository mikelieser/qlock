import QuartzCore
import ScreenSaver
import os

private let log = Logger(subsystem: "de.mikelieser.qlock", category: "saver")

/// The screensaver: the Rust engine (native/) rasterises sprites once and says every frame where each one
/// goes; Core Animation composites them on the GPU. No WebKit, so nothing gets frozen or discarded when
/// the host takes the view out of its window.
@objc(QlockView)
final class QlockView: ScreenSaverView {
  private var engine: OpaquePointer?
  private var images: [CGImage] = []
  private var layers: [CALayer] = []
  private var last: [QlockInstance] = []
  private let canvas = CALayer()
  private let faceLayer = CALayer()
  private var timer: Timer?
  private var built: (size: CGSize, day: Int, flags: UInt32, locale: String, theme: String)?
  private var optionsSheet: OptionsSheet?
  private let scale: CGFloat
  /// Last time the host built or started a clock; the host's start/stop calls are unreliable.
  private static var lastActivity = Date()

  override init?(frame: NSRect, isPreview: Bool) {
    scale = NSScreen.main?.backingScaleFactor ?? 2
    super.init(frame: frame, isPreview: isPreview)
    QlockView.lastActivity = Date()
    log.info("init frame=\(NSStringFromRect(frame), privacy: .public) preview=\(isPreview, privacy: .public)")
    animationTimeInterval = 3600
    wantsLayer = true
    layer?.backgroundColor = NSColor.black.cgColor
    canvas.isGeometryFlipped = true
    canvas.frame = bounds
    layer?.addSublayer(canvas)

    let timer = Timer(timeInterval: 1.0 / 60.0, repeats: true) { [weak self] _ in self?.tick() }
    RunLoop.main.add(timer, forMode: .common)
    self.timer = timer

    NotificationCenter.default.addObserver(self, selector: #selector(rebuild), name: Preferences.changed, object: nil)
    if !isPreview {
      DistributedNotificationCenter.default().addObserver(
        self, selector: #selector(willStop), name: Notification.Name("com.apple.screensaver.willstop"), object: nil)
    }
  }

  required init?(coder: NSCoder) { nil }

  deinit {
    timer?.invalidate()
    if let engine { qlock_engine_free(engine) }
  }

  @objc private func rebuild() {
    built = nil
  }

  /// (Re)creates the engine when size, settings or the date line change.
  private func ensureEngine() {
    let size = bounds.size
    let day = Calendar.current.ordinality(of: .day, in: .era, for: Date()) ?? 0
    let flags = Preferences.flags()
    let locale = Preferences.string("locale")
    let theme = Preferences.string("theme")
    if let b = built, b.size == size, b.day == day, b.flags == flags, b.locale == locale, b.theme == theme { return }
    guard size.width > 0, size.height > 0 else { return }
    built = (size, day, flags, locale, theme)

    if let engine { qlock_engine_free(engine) }
    let started = Date()
    let caption = Preferences.flag("date") ? Self.caption(lang: Options.shared.lang(of: locale)) : nil
    engine = qlock_engine_new(
      locale, theme, UInt32(size.width * scale), UInt32(size.height * scale), flags, caption)
    guard let engine else { return }

    images = (0..<qlock_sprite_count(engine)).map { i in
      var w: UInt32 = 0
      var h: UInt32 = 0
      let pixels = qlock_sprite(engine, i, &w, &h)!
      return Self.image(pixels, width: Int(w), height: Int(h))
    }

    var face = [Float](repeating: 0, count: 5)
    qlock_face(engine, &face)
    CATransaction.begin()
    CATransaction.setDisableActions(true)
    canvas.frame = bounds
    canvas.sublayers = nil
    faceLayer.sublayers = nil
    faceLayer.frame = CGRect(
      x: CGFloat(face[0]) / scale, y: CGFloat(face[1]) / scale, width: CGFloat(face[2]) / scale,
      height: CGFloat(face[3]) / scale)
    faceLayer.cornerRadius = CGFloat(face[4]) / scale
    faceLayer.masksToBounds = true
    faceLayer.isGeometryFlipped = false
    layers = []
    last = []
    CATransaction.commit()
    log.info("engine \(locale, privacy: .public)/\(theme, privacy: .public) \(self.images.count) sprites in \(Date().timeIntervalSince(started), privacy: .public)s")
  }

  private func tick() {
    ensureEngine()
    guard let engine else { return }
    let parts = Calendar.current.dateComponents([.hour, .minute], from: Date())
    var list: UnsafePointer<QlockInstance>?
    let count = Int(qlock_update(engine, UInt32(parts.hour ?? 0), UInt32(parts.minute ?? 0), CACurrentMediaTime(), &list))
    guard let list else { return }

    CATransaction.begin()
    CATransaction.setDisableActions(true)
    if layers.count != count { createLayers(list, count) }
    for i in 0..<count {
      let inst = list[i]
      if i < last.count, last[i] == inst { continue }
      let layer = layers[i]
      let origin = inst.group == 1 ? faceLayer.frame.origin : .zero
      layer.position = CGPoint(x: CGFloat(inst.x) / scale - origin.x, y: CGFloat(inst.y) / scale - origin.y)
      layer.opacity = inst.opacity
      layer.isHidden = inst.opacity < 0.002
      if inst.scale != 1 || inst.rotation != 0 {
        layer.setAffineTransform(
          CGAffineTransform(rotationAngle: CGFloat(inst.rotation)).scaledBy(x: CGFloat(inst.scale), y: CGFloat(inst.scale)))
      } else if i < last.count, last[i].scale != 1 || last[i].rotation != 0 {
        layer.setAffineTransform(.identity)
      }
    }
    last = Array(UnsafeBufferPointer(start: list, count: count))
    CATransaction.commit()
  }

  private func createLayers(_ list: UnsafePointer<QlockInstance>, _ count: Int) {
    var faceAdded = false
    layers = (0..<count).map { i in
      let inst = list[i]
      let image = images[Int(inst.sprite)]
      let layer = CALayer()
      layer.contents = image
      layer.contentsScale = scale
      layer.bounds = CGRect(x: 0, y: 0, width: CGFloat(image.width) / scale, height: CGFloat(image.height) / scale)
      if inst.group == 1 {
        if !faceAdded {
          canvas.addSublayer(faceLayer)
          faceAdded = true
        }
        faceLayer.addSublayer(layer)
      } else {
        canvas.addSublayer(layer)
      }
      return layer
    }
    last = []
  }

  private static func image(_ pixels: UnsafePointer<UInt8>, width: Int, height: Int) -> CGImage {
    let data = Data(bytes: pixels, count: width * height * 4) as CFData
    return CGImage(
      width: width, height: height, bitsPerComponent: 8, bitsPerPixel: 32, bytesPerRow: width * 4,
      space: CGColorSpace(name: CGColorSpace.sRGB)!,
      bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedLast.rawValue),
      provider: CGDataProvider(data: data)!, decode: nil, shouldInterpolate: true, intent: .defaultIntent)!
  }

  /// Short weekday and date like the web app: "So · 1. November".
  private static func caption(lang: String) -> String {
    let locale = Locale(identifier: lang)
    let weekday = DateFormatter()
    weekday.locale = locale
    weekday.dateFormat = "EEE"
    let date = DateFormatter()
    date.locale = locale
    date.setLocalizedDateFormatFromTemplate("dMMMM")
    let now = Date()
    let day = weekday.string(from: now).trimmingCharacters(in: CharacterSet(charactersIn: "."))
    return "\(day) · \(date.string(from: now))"
  }

  /// Ends the host process once the screensaver is really gone, which frees everything.
  /// Exiting right away is wrong: the host respawns during the fade-out and shows blank views.
  @objc private func willStop() {
    let stoppedAt = Date()
    DispatchQueue.main.asyncAfter(deadline: .now() + 10) {
      guard QlockView.lastActivity < stoppedAt else { return }
      log.info("screensaver gone, exiting host")
      exit(0)
    }
  }

  override func startAnimation() {
    super.startAnimation()
    QlockView.lastActivity = Date()
  }

  override func setFrameSize(_ newSize: NSSize) {
    super.setFrameSize(newSize)
    built = nil
  }

  override func draw(_ rect: NSRect) {}
  override func animateOneFrame() {}

  override var hasConfigureSheet: Bool { true }

  override var configureSheet: NSWindow? {
    let sheet = OptionsSheet()
    optionsSheet = sheet
    return sheet.window
  }
}

extension QlockInstance: @retroactive Equatable {
  public static func == (a: QlockInstance, b: QlockInstance) -> Bool {
    a.sprite == b.sprite && a.group == b.group && a.x == b.x && a.y == b.y && a.opacity == b.opacity
      && a.scale == b.scale && a.rotation == b.rotation
  }
}
