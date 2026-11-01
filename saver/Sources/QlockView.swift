import ScreenSaver
import WebKit
import os

private let log = Logger(subsystem: "de.mikelieser.qlock", category: "saver")

/// Shows the web clock as snapshots instead of a live web view.
///
/// The host (legacyScreenSaver) takes our view out of its window right after the start. WebKit then
/// treats the page as hidden: it freezes CSS animations and after a few seconds discards the layers,
/// which turned the Studio Display black. Snapshots of a hidden web view still render fine, so the web
/// view stays off-screen and we draw a fresh snapshot whenever the clock changes.
@objc(QlockView)
final class QlockView: ScreenSaverView, WKNavigationDelegate {
  private var webView: WKWebView?
  private var timer: Timer?
  private var lastKey = ""
  private var optionsSheet: OptionsSheet?
  /// Last time the host built or started a clock; the host's start/stop calls are unreliable.
  private static var lastActivity = Date()

  override init?(frame: NSRect, isPreview: Bool) {
    super.init(frame: frame, isPreview: isPreview)
    QlockView.lastActivity = Date()
    log.info("init frame=\(NSStringFromRect(frame), privacy: .public) preview=\(isPreview, privacy: .public)")
    animationTimeInterval = 3600
    wantsLayer = true
    layer?.backgroundColor = NSColor.black.cgColor
    layer?.contentsGravity = .resize

    let webView = WKWebView(frame: NSRect(origin: .zero, size: frame.size), configuration: WKWebViewConfiguration())
    webView.navigationDelegate = self
    self.webView = webView
    load()

    // Snapshot only when what's on screen can change: every second with the seconds frame, else per minute.
    let timer = Timer(timeInterval: 1, repeats: true) { [weak self] _ in self?.tick() }
    RunLoop.main.add(timer, forMode: .common)
    self.timer = timer

    NotificationCenter.default.addObserver(self, selector: #selector(load), name: Preferences.changed, object: nil)
    if !isPreview {
      DistributedNotificationCenter.default().addObserver(
        self, selector: #selector(willStop), name: Notification.Name("com.apple.screensaver.willstop"), object: nil)
    }
  }

  required init?(coder: NSCoder) { nil }

  deinit {
    timer?.invalidate()
  }

  @objc private func load() {
    guard let webView, let resources = Bundle(for: QlockView.self).resourceURL else { return }
    let page = resources.appendingPathComponent("web/index.html").absoluteURL
    var components = URLComponents(url: page, resolvingAgainstBaseURL: false)!
    components.queryItems = Preferences.queryItems()
    lastKey = ""
    webView.loadFileURL(components.url!, allowingReadAccessTo: resources)
  }

  override func setFrameSize(_ newSize: NSSize) {
    super.setFrameSize(newSize)
    webView?.setFrameSize(newSize)
    lastKey = ""
  }

  private func tick() {
    let parts = Calendar.current.dateComponents([.hour, .minute, .second], from: Date())
    let seconds = Preferences.flag("progress") ? parts.second ?? 0 : 0
    let key = "\(parts.hour ?? 0):\(parts.minute ?? 0):\(seconds)"
    guard key != lastKey else { return }
    lastKey = key
    snapshot()
  }

  private func snapshot() {
    guard let webView else { return }
    webView.evaluateJavaScript("window.qlockTick?.()") { [weak self] _, _ in
      let config = WKSnapshotConfiguration()
      config.snapshotWidth = NSNumber(value: Double(webView.bounds.width))
      webView.takeSnapshot(with: config) { image, error in
        guard let self, let image = image?.cgImage(forProposedRect: nil, context: nil, hints: nil) else {
          log.error("snapshot failed: \(String(describing: error), privacy: .public)")
          return
        }
        let scale = self.window?.backingScaleFactor ?? NSScreen.main?.backingScaleFactor ?? 2
        self.layer?.contentsScale = scale
        self.layer?.contents = image
      }
    }
  }

  func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
    // Fonts and the first frame need a moment after load.
    for delay in [0.3, 1.5] {
      DispatchQueue.main.asyncAfter(deadline: .now() + delay) { [weak self] in self?.snapshot() }
    }
  }

  func webViewWebContentProcessDidTerminate(_ webView: WKWebView) {
    log.error("web content process terminated, reloading")
    load()
  }

  /// Ends the host process once the screensaver is really gone, which frees every web view.
  /// Exiting right away is wrong: the host respawns during the fade-out and shows blank (white) views.
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

  override func draw(_ rect: NSRect) {}
  override func animateOneFrame() {}

  override var hasConfigureSheet: Bool { true }

  override var configureSheet: NSWindow? {
    let sheet = OptionsSheet()
    optionsSheet = sheet
    return sheet.window
  }
}
