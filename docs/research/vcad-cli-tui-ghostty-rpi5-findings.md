# vcad CLI TUI on Ghostty / Raspberry Pi 5 — Session Findings

Date: 2026-05-17

## Exact setup observed

Machine / OS:

- Hostname: `uconsole`
- Kernel: `Linux 6.12.78-v8-16k+ #4 SMP PREEMPT Fri Mar 27 00:36:23 EDT 2026`
- Architecture: `aarch64` / Debian architecture `arm64`
- Distribution: Ubuntu 24.04.4 LTS (`noble`)
- Device class: Raspberry Pi 5 / ARM64 environment

Terminal/session:

- Terminal: Ghostty 1.3.1, stable channel
- Ghostty package path: `/snap/ghostty/716/bin`
- Ghostty runtime: GTK app, OpenGL renderer
- `TERM=xterm-ghostty`
- `COLORTERM=truecolor`
- `GHOSTTY_BIN_DIR=/snap/ghostty/716/bin`
- `GHOSTTY_RESOURCES_DIR=/snap/ghostty/current/share/ghostty`
- Session: Wayland (`XDG_SESSION_TYPE=wayland`, `WAYLAND_DISPLAY=wayland-0`)
- Also has `DISPLAY=:0`
- Not running under tmux during the captured environment check

vcad / toolchain:

- vcad workspace version: `0.9.4`
- Git commit at time of investigation: `8b08d841`
- Rust: `rustc 1.95.0 (59807616e 2026-04-14)`
- Cargo: `cargo 1.95.0 (f2d3ce0bd 2026-03-21)`

## Symptoms reported

Running:

```bash
cargo run -p vcad-cli -- tui
```

or equivalent installed `vcad tui` in Ghostty showed several related rendering/input symptoms:

1. Initial TUI display looked mis-sized.
2. Most keys appeared not to work, except `q`.
3. Chat input and menus appeared underneath the viewport image.
4. Status/menu/welcome text accumulated or stacked over the viewport after starting a new project.
5. Chat/menu panels appeared to lack their expected gray/solid backgrounds.

A screenshot from `/home/ben/Pictures/Screenshots/Screenshot from 2026-05-17 21-05-01.png` showed the core issue clearly: the viewport image and text overlay layer were not compositing like a normal cell-based TUI. Old status/welcome/chat text remained visible over the viewport, and the chat panel looked transparent.

## Investigation summary

The TUI has two rendering modes:

- **Cell-native modes**: `HalfBlock` and `Braille`
  - The viewport and UI are both represented in the same terminal cell buffer.
  - Normal diff-based redraw works predictably.

- **Pixel-protocol modes**: `Kitty`, `iTerm2`, `Sixel`
  - The viewport is emitted as an image protocol escape sequence.
  - Menus/chat/status are emitted separately as normal terminal text.
  - This requires reliable terminal compositing between an image layer and text cells.

`crates/termview/src/terminal.rs` previously auto-detected Ghostty as Kitty-capable because `GHOSTTY_BIN_DIR` was set:

```rust
if env::var("GHOSTTY_BIN_DIR").is_ok() {
    return Self::kitty_with_tmux(false);
}
```

That meant Ghostty always entered the pixel-protocol path by default.

## Findings

The structural issue is that vcad's CLI TUI was defaulting Ghostty to the Kitty graphics protocol for a full-screen viewport while also expecting normal terminal text to behave as a stable overlay layer.

That assumption did not hold on this setup. Ghostty supports Kitty graphics, but the TUI's composition model exposed edge cases:

- Pixel viewport image dimensions were initially guessed from hardcoded cell size assumptions, causing sizing issues with Ghostty/font scaling.
- Viewport images could visually cover text overlays despite write order.
- Repeated image submissions could leave stale placements or stale text interactions.
- The text diff buffer did not initially know that the pixel image had overwritten prior overlay text.
- Terminal-theme `Default` backgrounds behaved effectively transparent over the image layer, so panels did not occlude the viewport as expected.

These were all symptoms of one larger design/configuration problem: **using a pixel image protocol as the default renderer in Ghostty for a UI that is architecturally cell-overlay based**.

## Fixes attempted during the session

Several targeted fixes improved pieces of the behavior:

1. Pixel viewport sizing
   - For Kitty/iTerm2/Sixel, prefer `crossterm::terminal::window_size()` pixel dimensions over guessed cell dimensions.
   - File: `crates/vcad-cli/src/app.rs`

2. Overlay repaint after pixel redraw
   - Added `CellBuffer::invalidate_all()` so text overlays repaint after a pixel viewport refresh.
   - Files:
     - `crates/vcad-cli/src/ui/buffer.rs`
     - `crates/vcad-cli/src/app.rs`

3. Kitty image z-index
   - Added `z=-1` to Kitty image transmission so the viewport is intended to sit behind text.
   - File: `crates/termview/src/protocols/kitty.rs`

4. Stable Kitty image id / delete before replace
   - Avoided incrementing the image id every frame.
   - Deleted/replaced the same image id to avoid stacking image placements.
   - Added `q=2` to suppress protocol responses.
   - Files:
     - `crates/termview/src/output.rs`
     - `crates/termview/src/protocols/kitty.rs`

5. Pixel overlay frame reset
   - Added `CellBuffer::begin_overlay_frame()` and called it before drawing text overlays in pixel-protocol mode.
   - This prevents stale overlay text from accumulating.
   - Files:
     - `crates/vcad-cli/src/ui/buffer.rs`
     - `crates/vcad-cli/src/app.rs`

6. Theme fallback
   - Tried deriving concrete dark terminal theme colors when OSC background probing fails, rather than using `Color::Default`.
   - File: `crates/vcad-cli/src/ui/theme.rs`

These patches improved behavior, especially menus/chat appearing above the viewport and reducing stale text stacking, but did not fully solve panel background compositing in Ghostty's Kitty path.

## Recommended structural fix

Default Ghostty to the cell-native `HalfBlock` renderer instead of Kitty graphics.

Implemented detection change:

```rust
if env::var("GHOSTTY_BIN_DIR").is_ok() {
    return Self {
        protocol: GraphicsProtocol::HalfBlock,
        true_color: true,
        width_px: None,
        height_px: None,
        cell_width: 8,
        cell_height: 16,
        in_tmux,
    };
}
```

File:

- `crates/termview/src/terminal.rs`

Rationale:

- `HalfBlock` keeps viewport and UI in one terminal cell buffer.
- It avoids image/text layering semantics entirely.
- It matches the TUI architecture better for interactive menus/chat/status overlays.
- Ghostty users can still opt into Kitty manually for experimentation:

```bash
TERMVIEW_PROTOCOL=kitty vcad tui
```

or from source:

```bash
TERMVIEW_PROTOCOL=kitty cargo run -p vcad-cli -- tui
```

## Workaround for current builds

Before the detection fix is released, force the stable cell renderer:

```bash
TERMVIEW_PROTOCOL=halfblock vcad tui
```

or from source:

```bash
TERMVIEW_PROTOCOL=halfblock cargo run -p vcad-cli -- tui
```

## Validation run during session

After changes, the following passed:

```bash
cargo fmt --check -p termview -p vcad-cli
cargo check -p vcad-cli
```

Warnings from sibling `tang` crates were present but unrelated to these changes.

## Changelog entry

A changelog entry was added:

- `changelog/entries/2026-05-17-cli-tui-ghostty-pixel-fix.json`

## Current modified files from this investigation

At the time of writing, relevant modified files were:

- `crates/termview/src/terminal.rs`
- `crates/termview/src/output.rs`
- `crates/termview/src/protocols/kitty.rs`
- `crates/vcad-cli/src/app.rs`
- `crates/vcad-cli/src/ui/buffer.rs`
- `crates/vcad-cli/src/ui/theme.rs`
- `changelog/entries/2026-05-17-cli-tui-ghostty-pixel-fix.json`

The key long-term behavioral change is the Ghostty default protocol selection in `crates/termview/src/terminal.rs`.
