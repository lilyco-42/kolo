# kolo

Zed 配置编辑器 — a GUI config editor for Zed.

Separates logic (`core`) from presentation (`gui`) so the UI layer can be swapped out.

## Quick Start

### Install via cargo-binstall

```bash
cargo install cargo-binstall
cargo binstall --git https://github.com/lilyco-42/kolo kolo
```

### Install from source

```bash
cargo install --git https://github.com/lilyco-42/kolo
```

### Download binaries

Pre-built binaries for Windows, macOS, and Linux are on the [Releases](https://github.com/lilyco-42/kolo/releases) page.

| Platform | Format |
|---|---|
| Windows | `.exe` / `.msi` / `.zip` |
| macOS (Intel) | `.tar.gz` |
| macOS (Apple Silicon) | `.tar.gz` |
| Linux | `.tar.gz` |

### Run

```bash
kolo
```

## Build

```bash
cargo build --release
```

## Structure

```
src/
├── main.rs    Entry point
├── core.rs    Config read/write logic
├── gui.rs     egui-based editor UI
assets/
├── simhei.ttf        Bundled CJK font
└── HarmonyOS_Sans_Medium.ttf  Fallback font
```

## License

MIT
