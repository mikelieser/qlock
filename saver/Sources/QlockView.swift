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
  private var webView: WKWebView!
  private var optionsSheet: OptionsSheet?

  override init?(frame: NSRect, isPreview: Bool) {
    super.init(frame: frame, isPreview: isPreview)
    log.info("init frame=\(NSStringFromRect(frame), privacy: .public) preview=\(isPreview, privacy: .public)")
    // The web app animates itself; no native drawing loop needed.
    animationTimeInterval = 3600
    wantsLayer = true
    layer?.backgroundColor = NSColor.black.cgColor

    webView = PassiveWebView(frame: bounds, configuration: WKWebViewConfiguration())
    webView.autoresizingMask = [.width, .height]
    webView.navigationDelegate = self
    addSubview(webView)
    load()

    NotificationCenter.default.addObserver(self, selector: #selector(load), name: Preferences.changed, object: nil)
    if !isPreview {
      // Since Sonoma the host process (legacyScreenSaver) often never calls stopAnimation and keeps
      // old instances alive, so web views pile up. Leaving the process on willstop is the usual fix.
      DistributedNotificationCenter.default().addObserver(
        self, selector: #selector(willStop), name: Notification.Name("com.apple.screensaver.willstop"), object: nil)
    }
  }

  required init?(coder: NSCoder) { nil }

  @objc private func load() {
    guard let resources = Bundle(for: QlockView.self).resourceURL else { return }
    let page = resources.appendingPathComponent("web/index.html").absoluteURL
    var components = URLComponents(url: page, resolvingAgainstBaseURL: false)!
    components.queryItems = Preferences.queryItems()
    webView.loadFileURL(components.url!, allowingReadAccessTo: resources)
  }

  @objc private func willStop() {
    log.info("willstop, exiting")
    exit(0)
  }

  override func viewDidMoveToWindow() {
    super.viewDidMoveToWindow()
    log.info("window scale=\(self.window?.backingScaleFactor ?? 0, privacy: .public) screen=\(self.window?.screen?.localizedName ?? "-", privacy: .public) bounds=\(NSStringFromRect(self.bounds), privacy: .public)")
  }

  override func startAnimation() {
    super.startAnimation()
    log.info("start")
  }

  override func stopAnimation() {
    super.stopAnimation()
    log.info("stop")
  }

  func webView(_ webView: WKWebView, didFinish navigation: WKNavigation!) {
    webView.evaluateJavaScript("[devicePixelRatio, innerWidth, innerHeight, document.title].join(' ')") { result, _ in
      log.info("loaded \(String(describing: result ?? "-"), privacy: .public)")
    }
    // The host never shows the page as visible; this tells whether the clock still ends up fully drawn.
    DispatchQueue.main.asyncAfter(deadline: .now() + 3) {
      let check = "[document.visibilityState, getComputedStyle(document.querySelector('.clock')).opacity, "
        + "getComputedStyle(document.querySelector('.clock')).filter, document.title].join(' | ')"
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
