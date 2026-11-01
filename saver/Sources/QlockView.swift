import ScreenSaver
import WebKit

/// Lets every mouse event fall through to the screensaver, which ends itself on input.
private final class PassiveWebView: WKWebView {
  override func hitTest(_ point: NSPoint) -> NSView? { nil }
}

@objc(QlockView)
final class QlockView: ScreenSaverView {
  private var webView: WKWebView!
  private var optionsSheet: OptionsSheet?

  override init?(frame: NSRect, isPreview: Bool) {
    super.init(frame: frame, isPreview: isPreview)
    // The web app animates itself; no native drawing loop needed.
    animationTimeInterval = 3600
    wantsLayer = true
    layer?.backgroundColor = NSColor.black.cgColor

    webView = PassiveWebView(frame: bounds, configuration: WKWebViewConfiguration())
    webView.autoresizingMask = [.width, .height]
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
    exit(0)
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
