# Branch feature / fix inventory

Compared against `main` (`f6a9df53`). This list groups the branch changes into unique upstreamable features or bug fixes, rather than individual commits/files.

> Note: this inventory includes the current uncommitted working-tree changes in `crates/vcad-cli/src/app.rs`, `crates/vcad-cli/src/input.rs`, and `crates/vcad-cli/src/ui/chat.rs`.

## Candidate upstream items

### 1. CLI TUI viewport PNG rendering

**Type:** Feature  
**User impact:** The TUI can export the current viewport as a PNG screenshot.

Adds a `render` / `screenshot` command that renders current evaluated geometry with the active camera to a PNG file, defaulting to `vcad-render.png`. It accepts an output path, optional dimensions like `1920x1080`, and `--no-open`; by default it attempts to open the rendered file with the OS default viewer.

**Related changes:**
- `crates/vcad-cli/src/app.rs`
- `crates/vcad-cli/src/ui/menu.rs`
- `crates/vcad-cli/src/ui/toolbar.rs`
- `crates/vcad-app/src/commands.rs`
- `crates/vcad-i18n/locales/en.json`
- `changelog/entries/2026-05-18-cli-tui-render-command.json`

### 2. Improved terminal viewport shading and outlines

**Type:** Feature / visual bug fix  
**User impact:** TUI-rendered models are easier to read; dark faces no longer look like missing geometry.

Changes the terminal rasterizer to use brighter CAD-style lighting with key/fill/head/rim contributions, updated default mesh/selection colors, and a silhouette/crease outline pass based on pick/depth buffers.

**Related changes:**
- `crates/termview/src/rasterize.rs`
- `crates/vcad-cli/src/app.rs`
- `changelog/entries/2026-05-18-cli-tui-viewport-colors.json`

### 3. Ground grid rendering fix

**Type:** Bug fix  
**User impact:** Viewport grid lines no longer become thick rectangles or visually cut through models at grazing camera angles.

Reworks grid rendering from depth-writing 3D quads into constant-width screen-space guide lines that do not write pick IDs or depth, so model geometry remains visually dominant.

**Related changes:**
- `crates/termview/src/rasterize.rs`

### 4. Slash commands inside the chat panel

**Type:** Feature  
**User impact:** Users can run TUI commands from chat by typing `/command`, while normal messages still go to the assistant.

Adds slash-command detection in chat input, routes non-empty slash messages through `App::process_command`, reports success/failure in the chat transcript, and opens chat seeded with `/` when the user presses `/` in normal mode.

**Related changes:**
- `crates/vcad-cli/src/input.rs`
- `crates/vcad-cli/src/tui/modes.rs`
- `crates/vcad-cli/src/ui/chat.rs`

### 5. Chat slash-command suggestions and Tab completion

**Type:** Feature  
**User impact:** Discoverability improves for command-driven TUI usage.

Adds an in-chat suggestion list for slash commands, including usage text and descriptions. Pressing Tab completes the first matching slash command.

**Related changes:**
- `crates/vcad-cli/src/ui/chat.rs`
- `crates/vcad-cli/src/input.rs`

### 6. Slash/command aliases

**Type:** Feature / usability improvement  
**User impact:** Common synonyms work for commands, especially from chat.

Adds aliases such as `tube` for cylinder, `ball` for sphere, `spin`/`turn` for rotate, `resize` for scale, `combine` for union, `cut` for difference, `flip` for mirror, `png`/`image` for render, `all` for select all, camera aliases like `iso`/`top`/`fit`, and `theme` for cycle theme.

**Related changes:**
- `crates/vcad-cli/src/app.rs`
- `crates/vcad-cli/src/ui/chat.rs`

### 7. Clear chat history command

**Type:** Feature / privacy cleanup  
**User impact:** Users can clear both visible and persisted TUI chat history.

Adds `clear_chat` / `chat_clear`, aborts any in-flight chat turn, clears visible chat state, clears session buffers, and removes persisted history on disk.

**Related changes:**
- `crates/vcad-cli/src/app.rs`
- `crates/vcad-cli/src/chat_session.rs`
- `crates/vcad-cli/src/ui/chat.rs`

### 8. Parts list keyboard focus and multi-selection UX

**Type:** Feature  
**User impact:** Selecting parts in the TUI is more precise and discoverable.

Adds `j`/`k` navigation through the parts list without changing selection, `Enter`/`Space` toggling of the focused part, explicit focus marker plus `[x]` selection indicator, and root node IDs shown in the sidebar.

**Related changes:**
- `crates/vcad-cli/src/input.rs`
- `crates/vcad-cli/src/ui/tree.rs`
- `crates/vcad-cli/src/tui/modes.rs`

### 9. Command-mode part selection by ID

**Type:** Feature  
**User impact:** Users can select exact parts from commands or slash commands.

Adds `select <id> [id...]`, validates IDs against root parts, updates the rendered selection highlight, and keeps `select_all` / `deselect` dirty-state handling consistent.

**Related changes:**
- `crates/vcad-cli/src/app.rs`
- `crates/vcad-cli/src/ui/tree.rs`

### 10. Selection redraw fixes

**Type:** Bug fix  
**User impact:** Viewport selection highlights update immediately after keyboard, mouse, or command selection changes.

Marks the render as dirty when selection changes via sidebar clicks, viewport picking, Tab cycling, Enter/Space toggling, `select`, `select_all`, or `deselect`.

**Related changes:**
- `crates/vcad-cli/src/app.rs`
- `crates/vcad-cli/src/input.rs`

### 11. Toolbar/sidebar layout corrections

**Type:** Bug fix / UI polish  
**User impact:** Sidebar hit-testing and rendering align correctly when a toolbar input row is visible.

Introduces toolbar height calculation that accounts for inline tool input, moves the sidebar below the toolbar, widens it for IDs and checkboxes, and updates mouse hit-testing to use the same computed geometry.

**Related changes:**
- `crates/vcad-cli/src/input.rs`
- `crates/vcad-cli/src/ui/mod.rs`
- `crates/vcad-cli/src/ui/toolbar.rs`
- `crates/vcad-cli/src/ui/tree.rs`

### 12. Slash command focus handoff for inline inputs

**Type:** Bug fix  
**User impact:** Commands like `/render` that open an inline toolbar input can be completed without chat trapping subsequent typing.

When a slash command opens `tool_input`, chat is unfocused/closed so keyboard input goes to the inline prompt.

**Related changes:**
- `crates/vcad-cli/src/input.rs`

### 13. Updated movement controls for TUI translation

**Type:** Behavior change / usability improvement  
**User impact:** Keyboard translation is split across horizontal-plane movement and vertical/depth movement.

Changes movement hints and bindings so WASD moves on the X/Z plane and PageUp/PageDown move on Y. The hotkey hint text was updated accordingly.

**Related changes:**
- `crates/vcad-cli/src/input.rs`
- `crates/vcad-cli/src/tui/modes.rs`
- `cheatsheet.md`

### 14. TUI cheat sheet

**Type:** Documentation  
**User impact:** Provides a quick reference for building/running the TUI and using startup, normal mode, camera, movement, command mode, chat, menus, and sketch mode.

**Related changes:**
- `cheatsheet.md`

## Other changes observed

- `Cargo.lock` bumps `phyz` from `0.3.0` to `0.3.1`. No direct code change in this branch explains a user-facing feature from that bump.
