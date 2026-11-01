import QuartzCore
import ScreenSaver
import os

private let log = Logger(subsystem: "de.mikelieser.qlock", category: "saver")

/// The screensaver: the Rust engine (native/) rasterises sprites once and says for any moment where each
/// one goes. Pushing that to Core Animation every frame stutters, because the host forwards our layer tree
/// to the screen process at its own pace. So we sample the engine a few seconds ahead and hand Core
/// Animation keyframe tracks; the render server then interpolates at the display's refresh rate.
@objc(QlockView)
final class QlockView: ScreenSaverView {
  private var engine: OpaquePointer?
  private var images: [CGImage] = []
  private var layers: [CALayer] = []
  /// End of the last sampled window (media time); the next window starts there.
  private var windowEnd: CFTimeInterval = 0
  private var windowKey = false
  /// Shifts the clock, for the bench to show a minute change on demand.
  @objc public var clockOffset: TimeInterval = 0
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

    let timer = Timer(timeInterval: 0.25, repeats: true) { [weak self] _ in self?.tick() }
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
    qlock_release_pixels(engine)

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
    windowEnd = 0
    CATransaction.commit()
    log.info("engine \(locale, privacy: .public)/\(theme, privacy: .public) \(self.images.count) sprites in \(Date().timeIntervalSince(started), privacy: .public)s")
  }

  /// Seconds per sampled window and samples per second; Core Animation interpolates linearly in between.
  private static let window: CFTimeInterval = 6
  private static let rate: Double = 30

  private func tick() {
    advance(to: CACurrentMediaTime())
  }

  /// Keeps the animation tracks filled up to `now` + a few seconds. Public for the offline bench
  /// (saver/bench), which drives it with simulated time.
  @objc public func advance(to now: CFTimeInterval) {
    ensureEngine()
    guard engine != nil else { return }
    // Queue the next window two seconds early; a first window starts right away.
    if windowEnd == 0 || windowEnd < now {
      windowEnd = now
      queueWindow(from: now)
    } else if windowEnd - now < 2 {
      queueWindow(from: windowEnd)
    }
  }

  /// Samples the engine over [start, start + window] and adds the tracks as animations starting at `start`.
  private func queueWindow(from start: CFTimeInterval) {
    guard let engine else { return }
    let started = CACurrentMediaTime()
    let steps = Int(Self.window * Self.rate)
    let wallStart = Date().addingTimeInterval(clockOffset + start - CACurrentMediaTime())
    // Track values of the previous window's last sample; this window must continue from exactly there.
    var samples: [[QlockInstance]] = []
    samples.reserveCapacity(steps + 1)
    for step in 0...steps {
      let dt = Double(step) / Self.rate
      let parts = Calendar.current.dateComponents([.hour, .minute], from: wallStart.addingTimeInterval(dt))
      var list: UnsafePointer<QlockInstance>?
      let count = Int(qlock_update(engine, UInt32(parts.hour ?? 0), UInt32(parts.minute ?? 0), start + dt, &list))
      guard let list else { return }
      samples.append(Array(UnsafeBufferPointer(start: list, count: count)))
    }
    let count = samples[0].count

    CATransaction.begin()
    CATransaction.setDisableActions(true)
    if layers.count != count { samples[0].withUnsafeBufferPointer { createLayers($0.baseAddress!, count) } }
    let key = windowKey ? "b" : "a"
    windowKey.toggle()
    let keyTimes = (0...steps).map { NSNumber(value: Double($0) / Double(steps)) }
    for i in 0..<count {
      let layer = layers[i]
      let track = samples.map { $0[i] }
      let origin = track[0].group == 1 ? faceLayer.frame.origin : .zero
      let point = { (inst: QlockInstance) in
        CGPoint(x: CGFloat(inst.x) / self.scale - origin.x, y: CGFloat(inst.y) / self.scale - origin.y)
      }
      let begin = layer.convertTime(start, from: nil)
      func animate(_ path: String, _ values: [Any]) {
        let anim = CAKeyframeAnimation(keyPath: path)
        anim.values = values
        anim.keyTimes = keyTimes
        anim.beginTime = begin
        anim.duration = Self.window
        anim.calculationMode = .linear
        // Without this Core Animation may step slow tracks at a low rate (measured ~10 Hz in the bench).
        if #available(macOS 12.0, *) {
          anim.preferredFrameRateRange = CAFrameRateRange(minimum: 30, maximum: 120, preferred: 60)
        }
        // No fill: until `begin` the previous window's track (added earlier, so lower priority) is still
        // running, and a backwards fill here would freeze the layer for the overlap.
        anim.fillMode = .removed
        layer.add(anim, forKey: path + key)
      }
      let last = track[track.count - 1]
      if track.contains(where: { $0.opacity != last.opacity }) { animate("opacity", track.map { $0.opacity }) }
      if track.contains(where: { $0.x != last.x || $0.y != last.y }) {
        animate("position", track.map { NSValue(point: point($0)) })
      }
      if track.contains(where: { $0.scale != last.scale }) { animate("transform.scale", track.map { CGFloat($0.scale) }) }
      // Model values: the window's first sample. Before `begin` they only show where the previous window
      // had no track, i.e. where the value didn't change and already equals this first sample.
      let first = track[0]
      layer.opacity = first.opacity
      layer.position = point(first)
      layer.setValue(CGFloat(first.scale), forKeyPath: "transform.scale")
    }
    CATransaction.commit()
    windowEnd = start + Self.window
    log.debug("window \(count) layers in \(CACurrentMediaTime() - started, privacy: .public)s")
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
