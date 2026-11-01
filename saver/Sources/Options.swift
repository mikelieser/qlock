import Foundation
import ScreenSaver

/// Option lists generated from the web app by `saver/options.ts`.
struct Options: Decodable {
  struct Item: Decodable { let id: String; let label: String }
  struct Group: Decodable { let group: String; let items: [Item] }
  struct Switch: Decodable { let key: String; let label: String; let `default`: Bool }

  let locales: [Group]
  let themes: [Group]
  let switches: [Switch]
  let defaults: [String: String]

  static let shared: Options = {
    let bundle = Bundle(for: QlockView.self)
    guard let url = bundle.url(forResource: "options", withExtension: "json"),
      let data = try? Data(contentsOf: url),
      let options = try? JSONDecoder().decode(Options.self, from: data)
    else { return Options(locales: [], themes: [], switches: [], defaults: [:]) }
    return options
  }()
}

/// The user's choices, kept in the screensaver's own defaults domain.
enum Preferences {
  static let changed = Notification.Name("de.mikelieser.qlock.preferences-changed")

  static var store: UserDefaults {
    ScreenSaverDefaults(forModuleWithName: Bundle(for: QlockView.self).bundleIdentifier ?? "de.mikelieser.qlock")
      ?? .standard
  }

  static func string(_ key: String) -> String {
    store.string(forKey: key) ?? Options.shared.defaults[key] ?? ""
  }

  static func bool(_ sw: Options.Switch) -> Bool {
    store.object(forKey: sw.key) as? Bool ?? sw.default
  }

  /// URL parameters for the web app; they take precedence over anything it stored itself.
  static func queryItems() -> [URLQueryItem] {
    var items = [
      URLQueryItem(name: "locale", value: string("locale")),
      URLQueryItem(name: "theme", value: string("theme")),
      URLQueryItem(name: "mode", value: "live"),
      URLQueryItem(name: "still", value: "1"),
      // Animated extras stay off; the web app would otherwise restore them from its own storage.
      URLQueryItem(name: "effect", value: "0"),
      URLQueryItem(name: "transition", value: "0"),
      URLQueryItem(name: "ambient", value: "0"),
    ]
    for sw in Options.shared.switches {
      items.append(URLQueryItem(name: sw.key, value: bool(sw) ? "1" : "0"))
    }
    return items.filter { !($0.value ?? "").isEmpty }
  }
}
