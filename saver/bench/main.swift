// Offline bench for QLOCK.saver: loads the built bundle, drives it with simulated time and renders exactly
// what Core Animation would show (CARenderer, animations included) into PNG frames, without touching the
// screen. Reports timing and how smoothly the background particles move.
//
// Usage: qlock-bench <QLOCK.saver> <out-dir> [key=value …]
//   theme=sunset locale=kob seconds=14 fps=60 size=1512x982 frames=png|none only=particles
import AppKit
import Metal
import QuartzCore
import ScreenSaver

let args = CommandLine.arguments
guard args.count >= 3 else {
  print("usage: qlock-bench <QLOCK.saver> <out-dir> [theme= locale= seconds= fps= size= frames= only=]")
  exit(2)
}
var opt = ["theme": "sunset", "locale": "kob", "seconds": "14", "fps": "60", "size": "1512x982", "frames": "png"]
for a in args.dropFirst(3) {
  let kv = a.split(separator: "=", maxSplits: 1).map(String.init)
  if kv.count == 2 { opt[kv[0]] = kv[1] }
}
let out = URL(fileURLWithPath: args[2], isDirectory: true)
try? FileManager.default.createDirectory(at: out, withIntermediateDirectories: true)

// Settings go into the saver's defaults domain, as the options sheet would write them.
let bundle = Bundle(path: args[1])!
let defaults = ScreenSaverDefaults(forModuleWithName: bundle.bundleIdentifier!)!
defaults.set(opt["theme"], forKey: "theme")
defaults.set(opt["locale"], forKey: "locale")
for key in ["date", "effect", "transition", "ambient", "dots", "intro"] { defaults.set(true, forKey: key) }
defaults.set(opt["stretch"] == "1", forKey: "stretch")
defaults.synchronize()

precondition(bundle.load(), "bundle load failed")
let cls = bundle.principalClass as! ScreenSaverView.Type
let size = opt["size"]!.split(separator: "x").map { CGFloat(Double($0)!) }
let scale: CGFloat = NSScreen.main?.backingScaleFactor ?? 2
let rect = NSRect(x: 0, y: 0, width: size[0], height: size[1])
let view = cls.init(frame: rect, isPreview: false)!
let root = view.layer!

let fps = Double(opt["fps"]!)!
let seconds = Double(opt["seconds"]!)!
let width = Int(size[0] * scale)
let height = Int(size[1] * scale)

let device = MTLCreateSystemDefaultDevice()!
let desc = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .bgra8Unorm, width: width, height: height, mipmapped: false)
desc.usage = [.renderTarget, .shaderRead]
desc.storageMode = .shared
let texture = device.makeTexture(descriptor: desc)!
// Same command queue for the renderer and our wait below, so reading the texture waits for the render.
let queue = device.makeCommandQueue()!
let renderer = CARenderer(mtlTexture: texture, options: [
  kCARendererColorSpace as String: CGColorSpace(name: CGColorSpace.sRGB)!,
  kCARendererMetalCommandQueue as String: queue,
])
renderer.bounds = CGRect(x: 0, y: 0, width: width, height: height)
// Render at pixel size: scale the root up by the backing factor.
let container = CALayer()
container.bounds = renderer.bounds
container.anchorPoint = .zero
container.position = .zero
container.isGeometryFlipped = false
root.anchorPoint = .zero
root.position = .zero
root.setAffineTransform(CGAffineTransform(scaleX: scale, y: scale))
container.addSublayer(root)
renderer.layer = container
// AppKit's own backing store of the view would show up as a tiny thumbnail otherwise.
root.contents = nil

let advanceSel = NSSelectorFromString("advanceTo:")
typealias Advance = @convention(c) (AnyObject, Selector, CFTimeInterval) -> Void
let advance = unsafeBitCast(view.method(for: advanceSel), to: Advance.self)

func timed(_ f: () -> Void) -> Double {
  let t = CACurrentMediaTime()
  f()
  return CACurrentMediaTime() - t
}

// minute-change=N: start N seconds before the next full minute, so the transition is in the clip.
if let lead = opt["minute-change"].flatMap(Double.init) {
  let now = Date().timeIntervalSince1970
  let next = (now / 60).rounded(.down) * 60 + 60
  view.setValue(next - lead - now, forKey: "clockOffset")
}
let t0 = CACurrentMediaTime()
let build = timed {
  advance(view, advanceSel, t0)
  CATransaction.flush()
}

// only=particles keeps the particle container; only=particle:N keeps a single particle, so its path can be
// followed frame by frame.
let single = opt["only"]?.hasPrefix("particle:") == true ? Int(opt["only"]!.dropFirst(9)) : nil
CATransaction.begin()
CATransaction.setDisableActions(true)
if opt["only"] == "particles" || single != nil {
  for l in root.sublayers?.first?.sublayers ?? [] where l.masksToBounds == false { l.isHidden = true }
  if let single, let face = root.sublayers?.first?.sublayers?.first(where: { $0.masksToBounds }) {
    for (i, l) in (face.sublayers ?? []).enumerated() where i != single { l.isHidden = true }
  }
}
CATransaction.commit()
CATransaction.flush()

var pixels = [UInt8](repeating: 0, count: width * height * 4)
var advanceTimes: [Double] = []
var renderTimes: [Double] = []
var centroids: [[(Double, Double)]] = []
let frames = Int(seconds * fps)

func saveFrame(_ index: Int) {
  let data = Data(pixels)
  let provider = CGDataProvider(data: data as CFData)!
  let image = CGImage(
    width: width, height: height, bitsPerComponent: 8, bitsPerPixel: 32, bytesPerRow: width * 4,
    space: CGColorSpace(name: CGColorSpace.sRGB)!,
    bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.premultipliedFirst.rawValue | CGBitmapInfo.byteOrder32Little.rawValue),
    provider: provider, decode: nil, shouldInterpolate: false, intent: .defaultIntent)!
  // The Metal texture is bottom-up; flip for viewing.
  let ctx = CGContext(data: nil, width: width, height: height, bitsPerComponent: 8, bytesPerRow: 0,
    space: CGColorSpace(name: CGColorSpace.sRGB)!, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)!
  ctx.translateBy(x: 0, y: CGFloat(height))
  ctx.scaleBy(x: 1, y: -1)
  ctx.draw(image, in: CGRect(x: 0, y: 0, width: width, height: height))
  let rep = NSBitmapImageRep(cgImage: ctx.makeImage()!)
  try! rep.representation(using: .png, properties: [:])!.write(to: out.appendingPathComponent(String(format: "f%04d.png", index)))
}

/// Bright blobs (particles on black) as centroids, on a 4× downsampled grid.
func blobs() -> [(Double, Double)] {
  let step = 4
  let gw = width / step, gh = height / step
  var lit = [Bool](repeating: false, count: gw * gh)
  for gy in 0..<gh {
    for gx in 0..<gw {
      let i = ((gy * step) * width + gx * step) * 4
      lit[gy * gw + gx] = Int(pixels[i]) + Int(pixels[i + 1]) + Int(pixels[i + 2]) > 120
    }
  }
  var seen = [Bool](repeating: false, count: gw * gh)
  var result: [(Double, Double)] = []
  for start in 0..<(gw * gh) where lit[start] && !seen[start] {
    var stack = [start]
    seen[start] = true
    var sx = 0.0, sy = 0.0, n = 0.0
    while let c = stack.popLast() {
      let x = c % gw, y = c / gw
      sx += Double(x); sy += Double(y); n += 1
      for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let nx = x + dx, ny = y + dy
        if nx >= 0, ny >= 0, nx < gw, ny < gh, lit[ny * gw + nx], !seen[ny * gw + nx] {
          seen[ny * gw + nx] = true
          stack.append(ny * gw + nx)
        }
      }
    }
    result.append((sx / n * Double(step), sy / n * Double(step)))
  }
  return result
}

let realtime = opt["realtime"] == "1"
for i in 0..<frames {
  let t = t0 + Double(i) / fps
  if realtime, t > CACurrentMediaTime() { Thread.sleep(forTimeInterval: t - CACurrentMediaTime()) }
  advanceTimes.append(timed {
    advance(view, advanceSel, t)
    // No run loop here: commit the layer changes by hand so the renderer sees them.
    CATransaction.flush()
  })
  renderTimes.append(timed {
    renderer.beginFrame(atTime: t, timeStamp: nil)
    renderer.addUpdate(renderer.bounds)
    renderer.render()
    renderer.endFrame()
    let cb = queue.makeCommandBuffer()!
    cb.commit()
    cb.waitUntilCompleted()
  })
  texture.getBytes(&pixels, bytesPerRow: width * 4, from: MTLRegionMake2D(0, 0, width, height), mipmapLevel: 0)
  if i == 0 {
    let lit = stride(from: 0, to: pixels.count, by: 4).filter { pixels[$0 + 3] > 0 }.count
    print("first frame: \(lit * 100 / (width * height)) % of pixels drawn")
  }
  if single != nil {
    // Brightness-weighted centroid of the only thing on screen, sub-pixel precise.
    var sx = 0.0, sy = 0.0, sw = 0.0
    for y in stride(from: 0, to: height, by: 2) {
      for x in stride(from: 0, to: width, by: 2) {
        let i = (y * width + x) * 4
        let v = Double(Int(pixels[i]) + Int(pixels[i + 1]) + Int(pixels[i + 2]))
        if v > 30 { sx += Double(x) * v; sy += Double(y) * v; sw += v }
      }
    }
    centroids.append(sw > 8000 ? [(sx / sw, sy / sw)] : [])
  } else if opt["only"] == "particles" {
    centroids.append(blobs())
  }
  if opt["frames"] == "png" { saveFrame(i) }
}

func stats(_ xs: [Double]) -> String {
  let s = xs.sorted()
  let p = { (q: Double) in s[min(s.count - 1, Int(Double(s.count) * q))] * 1000 }
  return String(format: "median %.2f ms, p95 %.2f ms, max %.2f ms", p(0.5), p(0.95), s.last! * 1000)
}

var report = """
  engine build + first window: \(String(format: "%.0f", build * 1000)) ms
  advance per frame: \(stats(advanceTimes))
  CARenderer per frame: \(stats(renderTimes))

  """

if !centroids.isEmpty, centroids.contains(where: { !$0.isEmpty }) {
  // Match each blob to the nearest blob in the previous frame; big moves mean a jump or reset.
  var moves: [Double] = []
  var jumps: [(Int, Double)] = []
  for f in 1..<centroids.count {
    for (x, y) in centroids[f] {
      let d = centroids[f - 1].map { hypot($0.0 - x, $0.1 - y) }.min() ?? .infinity
      if d.isFinite {
        moves.append(d)
        if d > 12 { jumps.append((f, d)) }
      }
    }
  }
  let sorted = moves.isEmpty ? [0] : moves.sorted()
  report += String(
    format: "particles: %d blobs/frame avg, move median %.2f px, p99 %.2f px, max %.2f px\n",
    centroids.map(\.count).reduce(0, +) / centroids.count, sorted[sorted.count / 2], sorted[Int(Double(sorted.count) * 0.99)],
    sorted.last ?? 0)
  report += "jumps > 12 px: \(jumps.count)" + (jumps.isEmpty ? "" : " at frames \(jumps.prefix(12).map { "\($0.0) (\(Int($0.1)) px)" })") + "\n"
}
if single != nil {
  // One particle: steps between consecutive visible frames; a smooth rise is a few px per frame.
  var steps: [(Int, Double)] = []
  for f in 1..<centroids.count where centroids[f].count == 1 && centroids[f - 1].count == 1 {
    steps.append((f, hypot(centroids[f][0].0 - centroids[f - 1][0].0, centroids[f][0].1 - centroids[f - 1][0].1)))
  }
  let visible = centroids.filter { $0.count == 1 }.count
  let sortedSteps = steps.map(\.1).sorted()
  report += String(format: "single particle: visible %d/%d frames, step median %.2f px, max %.2f px\n", visible,
    centroids.count, sortedSteps.isEmpty ? 0 : sortedSteps[sortedSteps.count / 2], sortedSteps.last ?? 0)
  let big = steps.filter { $0.1 > 12 }
  report += "big steps: \(big.map { "f\($0.0): \(Int($0.1)) px" })\n"
  let still = steps.filter { $0.1 < 0.05 }.count
  report += "frames without movement while visible: \(still)\n"
}
print(report, terminator: "")
try report.write(to: out.appendingPathComponent("report.txt"), atomically: true, encoding: .utf8)
exit(0)
