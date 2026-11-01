import AppKit

/// The "Optionen …" dialog in System Settings: language, theme and the on/off switches.
final class OptionsSheet: NSObject {
  let window: NSWindow
  private let localePopup = NSPopUpButton()
  private let themePopup = NSPopUpButton()
  private var checkboxes: [(Options.Switch, NSButton)] = []

  override init() {
    window = NSWindow(
      contentRect: NSRect(x: 0, y: 0, width: 380, height: 10), styleMask: [.titled], backing: .buffered, defer: true)
    super.init()

    let options = Options.shared
    fill(localePopup, with: options.locales, selected: Preferences.string("locale"))
    fill(themePopup, with: options.themes, selected: Preferences.string("theme"))

    let grid = NSGridView(views: [
      [label("Sprache"), localePopup],
      [label("Theme"), themePopup],
    ])
    grid.column(at: 0).xPlacement = .trailing
    grid.rowSpacing = 10
    grid.rowAlignment = .firstBaseline

    let switches = NSStackView()
    switches.orientation = .vertical
    switches.alignment = .leading
    switches.spacing = 6
    for sw in options.switches {
      let box = NSButton(checkboxWithTitle: sw.label, target: nil, action: nil)
      box.state = Preferences.bool(sw) ? .on : .off
      checkboxes.append((sw, box))
      switches.addArrangedSubview(box)
    }

    let cancel = NSButton(title: "Abbrechen", target: self, action: #selector(close))
    cancel.keyEquivalent = "\u{1b}"
    let ok = NSButton(title: "OK", target: self, action: #selector(save))
    ok.keyEquivalent = "\r"
    let buttons = NSStackView(views: [NSView(), cancel, ok])
    buttons.setHuggingPriority(.defaultLow, for: .horizontal)

    let content = NSStackView(views: [grid, switches, buttons])
    content.orientation = .vertical
    content.alignment = .leading
    content.spacing = 18
    content.edgeInsets = NSEdgeInsets(top: 20, left: 20, bottom: 20, right: 20)
    buttons.widthAnchor.constraint(equalTo: content.widthAnchor, constant: -40).isActive = true
    localePopup.widthAnchor.constraint(greaterThanOrEqualToConstant: 240).isActive = true
    themePopup.widthAnchor.constraint(equalTo: localePopup.widthAnchor).isActive = true
    window.contentView = content
  }

  private func label(_ text: String) -> NSTextField {
    NSTextField(labelWithString: text)
  }

  /// Groups as disabled headings with their entries indented below, like the pickers in the web app.
  private func fill(_ popup: NSPopUpButton, with groups: [Options.Group], selected: String) {
    let menu = NSMenu()
    menu.autoenablesItems = false
    for group in groups {
      if menu.numberOfItems > 0 { menu.addItem(.separator()) }
      let heading = NSMenuItem(title: group.group, action: nil, keyEquivalent: "")
      heading.isEnabled = false
      menu.addItem(heading)
      for item in group.items {
        let entry = NSMenuItem(title: item.label, action: nil, keyEquivalent: "")
        entry.representedObject = item.id
        entry.indentationLevel = 1
        menu.addItem(entry)
      }
    }
    popup.menu = menu
    if let item = menu.items.first(where: { $0.representedObject as? String == selected }) {
      popup.select(item)
    }
  }

  @objc private func save() {
    let store = Preferences.store
    if let id = localePopup.selectedItem?.representedObject as? String { store.set(id, forKey: "locale") }
    if let id = themePopup.selectedItem?.representedObject as? String { store.set(id, forKey: "theme") }
    for (sw, box) in checkboxes { store.set(box.state == .on, forKey: sw.key) }
    store.synchronize()
    NotificationCenter.default.post(name: Preferences.changed, object: nil)
    close()
  }

  @objc private func close() {
    if let parent = window.sheetParent {
      parent.endSheet(window)
    } else {
      window.close()
    }
  }
}
