import ScreenSaver
import WebKit
import os

private let log = Logger(subsystem: "de.mikelieser.qlock", category: "saver")

/// Lets every mouse event fall through to the screensaver, which ends itself on input.
private final class PassiveWebView: WKWebView {
  override func hitTest(_ point: NSPoint) -> NSView? { nil }
}

@objc(QlockView)
final class QlockView: ScreenSaverView, WKNavigationDelegate {
  private var webView: WKWebView?
  private var optionsSheet: OptionsSheet?
  /// Last time the host built or started a clock; the host's start/stop calls are unreliable.
  private static var lastActivity = Date()

  override init?(frame: NSRect, isPreview: Bool) {
    super.init(frame: frame, isPreview: isPreview)
    QlockView.lastActivity = Date()
    log.info("init frame=\(NSStringFromRect(frame), privacy: .public) preview=\(isPreview, privacy: .public)")
    // The web app animates itself; no native drawing loop needed.
    animationTimeInterval = 3600
    wantsLayer = true
    layer?.backgroundColor = NSColor.black.cgColor
    showClock()

    NotificationCenter.default.addObserver(self, selector: #selector(load), name: Preferences.changed, object: nil)
    if !isPreview {
      // Since Sonoma the host (legacyScreenSaver) keeps old instances alive and calls start/stop
      // unreliably, so web views would pile up. See willStop.
      DistributedNotificationCenter.default().addObserver(
        self, selector: #selector(willStop), name: Notification.Name("com.apple.screensaver.willstop"), object: nil)
    }
  }

  required init?(coder: NSCoder) { nil }

  private func showClock() {
    guard webView == nil else { return }
    let view = PassiveWebView(frame: bounds, configuration: WKWebViewConfiguration())
    view.autoresizingMask = [.width, .height]
    view.navigationDelegate = self
    // Black instead of white while the page loads.
    if view.responds(to: NSSelectorFromString("_setDrawsBackground:")) {
      view.setValue(false, forKey: "drawsBackground")
    }
    addSubview(view)
    webView = view
    load()
  }

  @objc private func load() {
    guard let webView, let resources = Bundle(for: QlockView.self).resourceURL else { return }
    let page = resources.appendingPathComponent("web/index.html").absoluteURL
    var components = URLComponents(url: page, resolvingAgainstBaseURL: false)!
    components.queryItems = Preferences.queryItems()
    webView.loadFileURL(components.url!, allowingReadAccessTo: resources)
  }

  /// Ends the host process once the screensaver is really gone, which frees every web view.
  /// Exiting right away is wrong: the host respawns during the fade-out and shows blank (white) views.
  /// Dropping single web views is wrong too: the host sometimes keeps showing an older instance.
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
    showClock()
  }

  override func stopAnimation() {
    super.stopAnimation()
  }

  func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
    // The host never reports the page as visible; this shows whether the clock still ends up fully drawn.
    DispatchQueue.main.asyncAfter(deadline: .now() + 3) {
      let check = "[document.visibilityState, getComputedStyle(document.querySelector('.clock')).opacity, document.title].join(' | ')"
      webView.evaluateJavaScript(check) { result, _ in
        log.info("after 3s \(String(describing: result ?? "-"), privacy: .public)")
      }
    }
  }

  func webViewWebContentProcessDidTerminate(_ webView: WKWebView) {
    log.error("web content process terminated, reloading")
    load()
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
