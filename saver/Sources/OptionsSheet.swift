import AppKit
import ScreenSaver

/// The "Optionen …" sheet in System Settings: a live preview on top, language, the themes as thumbnails
/// by group (like the web app's picker), then what is shown and what moves. Changes show in the
/// preview at once; OK saves them, Cancel throws them away.
final class OptionsSheet: NSObject {
  let window: NSWindow
  private var draft: [String: Any] = [:]
  private let localePopup = NSPopUpButton()
  private let groupControl = NSSegmentedControl()
  private let groupNote = NSTextField(labelWithString: "")
  private let tileGrid = NSStackView()
  private var tiles: [String: ThemeTile] = [:]
  private var checkboxes: [(Options.Switch, NSButton)] = []
  private var preview: QlockView?
  private let width: CGFloat = 560
  private static var thumbnails: [String: NSImage] = [:]

  override init() {
    window = NSWindow(
      contentRect: NSRect(x: 0, y: 0, width: 600, height: 10), styleMask: [.titled], backing: .buffered, defer: true)
    super.init()

    let options = Options.shared
    draft["locale"] = Preferences.string("locale")
    draft["theme"] = Preferences.string("theme")
    for sw in options.switches { draft[sw.key] = Preferences.bool(sw) }
    Preferences.draft = draft

    let content = NSStackView()
    content.orientation = .vertical
    content.alignment = .leading
    content.spacing = 14
    content.edgeInsets = NSEdgeInsets(top: 20, left: 20, bottom: 20, right: 20)

    content.addArrangedSubview(makePreview())

    fill(localePopup, with: options.locales, selected: draft["locale"] as? String ?? "")
    localePopup.target = self
    localePopup.action = #selector(localeChanged)
    content.addArrangedSubview(row(label: "Sprache", localePopup))

    content.addArrangedSubview(heading("Theme"))
    groupControl.segmentCount = options.themes.count
    for (i, group) in options.themes.enumerated() {
      groupControl.setLabel(group.group, forSegment: i)
      groupControl.setWidth(0, forSegment: i)
    }
    groupControl.segmentStyle = .rounded
    groupControl.trackingMode = .selectOne
    groupControl.target = self
    groupControl.action = #selector(groupChanged)
    content.addArrangedSubview(groupControl)
    groupNote.font = .systemFont(ofSize: NSFont.smallSystemFontSize)
    groupNote.textColor = .secondaryLabelColor
    content.addArrangedSubview(groupNote)
    content.setCustomSpacing(6, after: groupControl)
    tileGrid.orientation = .vertical
    tileGrid.alignment = .leading
    tileGrid.spacing = 10
    content.addArrangedSubview(tileGrid)
    let current = draft["theme"] as? String ?? ""
    groupControl.selectedSegment = options.themes.firstIndex { $0.items.contains { $0.id == current } } ?? 0
    showGroup(groupControl.selectedSegment)

    let sections = [
      ("Anzeige", "Was auf der Uhr zu sehen ist."),
      ("Bewegung", "Was sich bewegt – jedes Theme hat dafür seine eigene Art."),
    ]
    for (group, note) in sections {
      content.addArrangedSubview(heading(group, note: note))
      let boxes = options.switches.filter { $0.group == group }.map { sw -> NSButton in
        let box = NSButton(checkboxWithTitle: sw.label, target: self, action: #selector(switchChanged(_:)))
        box.state = (draft[sw.key] as? Bool ?? sw.default) ? .on : .off
        box.toolTip = sw.hint
        checkboxes.append((sw, box))
        return box
      }
      content.addArrangedSubview(columns(boxes, perRow: 3))
    }

    let reset = NSButton(title: "Standard", target: self, action: #selector(resetToDefaults))
    reset.toolTip = "Alle Einstellungen auf den Anfangszustand, Sprache bleibt."
    let cancel = NSButton(title: "Abbrechen", target: self, action: #selector(close))
    cancel.keyEquivalent = "\u{1b}"
    let ok = NSButton(title: "OK", target: self, action: #selector(save))
    ok.keyEquivalent = "\r"
    let buttons = NSStackView(views: [reset, NSView(), cancel, ok])
    content.addArrangedSubview(buttons)
    buttons.widthAnchor.constraint(equalToConstant: width).isActive = true

    window.contentView = content
    // fittingSize leaves out the trailing inset of a leading-aligned stack; set the width ourselves.
    window.setContentSize(NSSize(width: width + 40, height: content.fittingSize.height))
  }

  // MARK: Layout helpers

  private func makePreview() -> NSView {
    let screen = NSScreen.main?.frame.size ?? CGSize(width: 16, height: 10)
    let height = (width * screen.height / screen.width).rounded()
    // Sized from the start: the clock inside resizes with the box, and growing from zero would double it.
    let box = NSView(frame: NSRect(x: 0, y: 0, width: width, height: height))
    box.wantsLayer = true
    box.layer?.cornerRadius = 8
    box.layer?.masksToBounds = true
    box.layer?.backgroundColor = NSColor.black.cgColor
    box.translatesAutoresizingMaskIntoConstraints = false
    box.widthAnchor.constraint(equalToConstant: width).isActive = true
    box.heightAnchor.constraint(equalToConstant: height).isActive = true
    if let view = QlockView(frame: NSRect(x: 0, y: 0, width: width, height: height), isPreview: true) {
      view.autoresizingMask = [.width, .height]
      box.addSubview(view)
      view.startAnimation()
      preview = view
    }
    // Watch the words change without waiting for the next minute.
    let replay = NSButton(title: "Minutenwechsel zeigen", target: self, action: #selector(showMinuteChange))
    replay.bezelStyle = .accessoryBarAction
    replay.controlSize = .small
    replay.toolTip = "Spielt den nächsten Minutenwechsel in der Vorschau sofort ab."
    replay.translatesAutoresizingMaskIntoConstraints = false
    box.addSubview(replay)
    NSLayoutConstraint.activate([
      replay.centerXAnchor.constraint(equalTo: box.centerXAnchor),
      replay.bottomAnchor.constraint(equalTo: box.bottomAnchor, constant: -10),
    ])
    return box
  }

  private func heading(_ text: String, note: String? = nil) -> NSView {
    let label = NSTextField(labelWithString: text)
    label.font = .boldSystemFont(ofSize: NSFont.systemFontSize)
    guard let note else { return label }
    let detail = NSTextField(labelWithString: note)
    detail.font = .systemFont(ofSize: NSFont.smallSystemFontSize)
    detail.textColor = .secondaryLabelColor
    let row = NSStackView(views: [label, detail])
    row.spacing = 8
    row.alignment = .firstBaseline
    return row
  }

  private func row(label text: String, _ control: NSView) -> NSView {
    let label = NSTextField(labelWithString: text)
    let row = NSStackView(views: [label, control])
    row.spacing = 10
    control.widthAnchor.constraint(greaterThanOrEqualToConstant: 260).isActive = true
    return row
  }

  private func columns(_ views: [NSView], perRow: Int) -> NSView {
    let rows = stride(from: 0, to: views.count, by: perRow).map { start -> [NSView] in
      let slice = Array(views[start..<min(start + perRow, views.count)])
      return slice + Array(repeating: NSGridCell.emptyContentView, count: perRow - slice.count)
    }
    let grid = NSGridView(views: rows)
    grid.rowSpacing = 6
    grid.columnSpacing = 18
    for c in 0..<perRow { grid.column(at: c).width = (width - CGFloat(perRow - 1) * 18) / CGFloat(perRow) }
    return grid
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

  // MARK: Themes

  private func showGroup(_ index: Int) {
    groupNote.stringValue = Options.shared.themes[index].note ?? ""
    tileGrid.arrangedSubviews.forEach { $0.removeFromSuperview() }
    tiles = [:]
    let items = Options.shared.themes[index].items
    let perRow = 6
    let tileWidth = (width - CGFloat(perRow - 1) * 8) / CGFloat(perRow)
    for start in stride(from: 0, to: items.count, by: perRow) {
      let line = NSStackView()
      line.spacing = 8
      for item in items[start..<min(start + perRow, items.count)] {
        let tile = ThemeTile(id: item.id, label: item.label, width: tileWidth, target: self, action: #selector(themeChosen(_:)))
        tile.isSelected = item.id == draft["theme"] as? String
        tiles[item.id] = tile
        line.addArrangedSubview(tile)
        loadThumbnail(for: tile)
      }
      tileGrid.addArrangedSubview(line)
    }
    // Keep the sheet's height steady between groups: always room for two rows.
    if items.count <= perRow {
      let spacer = NSView()
      spacer.heightAnchor.constraint(equalToConstant: ThemeTile.height(width: tileWidth)).isActive = true
      tileGrid.addArrangedSubview(spacer)
    }
  }

  /// Thumbnails are rendered by the same Rust engine, in the background, once per sheet session.
  private func loadThumbnail(for tile: ThemeTile) {
    let key = tile.id + "|" + (draft["locale"] as? String ?? "")
    if let image = Self.thumbnails[key] {
      tile.image = image
      return
    }
    let locale = draft["locale"] as? String ?? "de"
    let scale = NSScreen.main?.backingScaleFactor ?? 2
    let size = tile.imageSize
    let (w, h) = (Int(size.width * scale), Int(size.height * scale))
    let id = tile.id
    let flags = UInt32(QLOCK_INTRO | QLOCK_DOTS)
    DispatchQueue.global(qos: .userInitiated).async {
      var pixels = [UInt8](repeating: 0, count: w * h * 4)
      let ok = qlock_snapshot(locale, id, UInt32(w), UInt32(h), flags, 10, 25, &pixels)
      guard ok, let provider = CGDataProvider(data: Data(pixels) as CFData),
        let cg = CGImage(
          width: w, height: h, bitsPerComponent: 8, bitsPerPixel: 32, bytesPerRow: w * 4,
          space: CGColorSpace(name: CGColorSpace.sRGB)!,
          bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedLast.rawValue), provider: provider,
          decode: nil, shouldInterpolate: true, intent: .defaultIntent)
      else { return }
      let image = NSImage(cgImage: cg, size: size)
      DispatchQueue.main.async {
        Self.thumbnails[key] = image
        if self.tiles[id] === tile { tile.image = image }
      }
    }
  }

  // MARK: Actions

  private func changed() {
    Preferences.draft = draft
    NotificationCenter.default.post(name: Preferences.changed, object: nil)
  }

  @objc private func localeChanged() {
    if let id = localePopup.selectedItem?.representedObject as? String { draft["locale"] = id }
    changed()
    showGroup(groupControl.selectedSegment)
  }

  @objc private func groupChanged() {
    showGroup(groupControl.selectedSegment)
  }

  @objc private func themeChosen(_ sender: ThemeTile) {
    draft["theme"] = sender.id
    for (id, tile) in tiles { tile.isSelected = id == sender.id }
    changed()
  }

  @objc private func switchChanged(_ sender: NSButton) {
    guard let sw = checkboxes.first(where: { $0.1 === sender })?.0 else { return }
    draft[sw.key] = sender.state == .on
    changed()
  }

  @objc private func showMinuteChange() {
    preview?.showMinuteChange()
  }

  @objc private func resetToDefaults() {
    let options = Options.shared
    draft["theme"] = options.defaults["theme"] ?? "formrausch"
    for (sw, box) in checkboxes {
      draft[sw.key] = sw.default
      box.state = sw.default ? .on : .off
    }
    let current = draft["theme"] as? String ?? ""
    groupControl.selectedSegment = options.themes.firstIndex { $0.items.contains { $0.id == current } } ?? 0
    showGroup(groupControl.selectedSegment)
    changed()
  }

  @objc private func save() {
    let store = Preferences.store
    for (key, value) in draft { store.set(value, forKey: key) }
    store.synchronize()
    finish()
  }

  @objc private func close() {
    finish()
  }

  private func finish() {
    Preferences.draft = nil
    NotificationCenter.default.post(name: Preferences.changed, object: nil)
    preview?.stopAnimation()
    preview?.removeFromSuperview()
    preview = nil
    if let parent = window.sheetParent {
      parent.endSheet(window)
    } else {
      window.close()
    }
  }
}

/// One theme in the picker: thumbnail with its name, an accent ring when chosen.
final class ThemeTile: NSControl {
  let id: String
  private let imageView = NSImageView()
  private let caption: NSTextField
  let imageSize: NSSize

  static func height(width: CGFloat) -> CGFloat { (width * 0.62).rounded() + 20 }

  var image: NSImage? {
    get { imageView.image }
    set { imageView.image = newValue }
  }

  var isSelected = false {
    didSet {
      imageView.layer?.borderWidth = isSelected ? 2.5 : 0.5
      imageView.layer?.borderColor = isSelected ? NSColor.controlAccentColor.cgColor : NSColor.separatorColor.cgColor
      caption.textColor = isSelected ? .labelColor : .secondaryLabelColor
    }
  }

  init(id: String, label: String, width: CGFloat, target: AnyObject, action: Selector) {
    self.id = id
    imageSize = NSSize(width: width, height: (width * 0.62).rounded())
    caption = NSTextField(labelWithString: label)
    super.init(frame: NSRect(x: 0, y: 0, width: width, height: Self.height(width: width)))
    self.target = target
    self.action = action
    imageView.wantsLayer = true
    imageView.layer?.cornerRadius = 6
    imageView.layer?.masksToBounds = true
    imageView.layer?.backgroundColor = NSColor.quaternaryLabelColor.cgColor
    imageView.imageScaling = .scaleAxesIndependently
    caption.font = .systemFont(ofSize: NSFont.smallSystemFontSize)
    caption.alignment = .center
    caption.lineBreakMode = .byTruncatingTail
    for v in [imageView, caption] {
      v.translatesAutoresizingMaskIntoConstraints = false
      addSubview(v)
    }
    NSLayoutConstraint.activate([
      widthAnchor.constraint(equalToConstant: width),
      heightAnchor.constraint(equalToConstant: Self.height(width: width)),
      imageView.topAnchor.constraint(equalTo: topAnchor),
      imageView.leadingAnchor.constraint(equalTo: leadingAnchor),
      imageView.widthAnchor.constraint(equalToConstant: imageSize.width),
      imageView.heightAnchor.constraint(equalToConstant: imageSize.height),
      caption.topAnchor.constraint(equalTo: imageView.bottomAnchor, constant: 3),
      caption.leadingAnchor.constraint(equalTo: leadingAnchor),
      caption.trailingAnchor.constraint(equalTo: trailingAnchor),
    ])
    isSelected = false
  }

  required init?(coder: NSCoder) { nil }

  override func mouseDown(with event: NSEvent) {
    sendAction(action, to: target)
  }
}
