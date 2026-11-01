import Foundation
import ScreenSaver

/// Option lists generated from the web app by `saver/options.ts`.
struct Options: Decodable {
  struct Item: Decodable { let id: String; let label: String; let lang: String? }
  struct Group: Decodable { let group: String; let items: [Item] }
  struct Switch: Decodable { let key: String; let label: String; let group: String; let `default`: Bool }

  let locales: [Group]
  let themes: [Group]
  let switches: [Switch]
  let defaults: [String: String]

  func lang(of locale: String) -> String {
    locales.flatMap(\.items).first { $0.id == locale }?.lang ?? "de-DE"
  }

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

  /// Unsaved choices while the options sheet is open; every clock in the process (the sheet's own
  /// preview and the small one in System Settings) shows them right away.
  static var draft: [String: Any]?

  static func string(_ key: String) -> String {
    draft?[key] as? String ?? store.string(forKey: key) ?? Options.shared.defaults[key] ?? ""
  }

  static func bool(_ sw: Options.Switch) -> Bool {
    draft?[sw.key] as? Bool ?? store.object(forKey: sw.key) as? Bool ?? sw.default
  }

  static func flag(_ key: String) -> Bool {
    Options.shared.switches.first { $0.key == key }.map(bool) ?? false
  }

  /// Option bits for `qlock_engine_new`.
  static func flags() -> UInt32 {
    var flags: UInt32 = 0
    for (key, bit) in [
      ("intro", QLOCK_INTRO), ("dots", QLOCK_DOTS), ("stretch", QLOCK_STRETCH), ("effect", QLOCK_EFFECT),
      ("transition", QLOCK_TRANSITION), ("ambient", QLOCK_AMBIENT), ("progress", QLOCK_PROGRESS),
    ] where flag(key) {
      flags |= UInt32(bit)
    }
    return flags
  }
}
