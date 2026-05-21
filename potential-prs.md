# Potential PR breakdown for `stuff` vs `main`

Reviewed range: `main..HEAD` (`3bd6cd66` through `9bd69c1b`). The branch contains several separable TUI, terminal-rendering, and chat-provider changes. The final branch state intentionally removes temporary docs, an accidental PNG, and branch changelog entries, so those are listed as cleanup rather than candidate feature PRs.

## Commit inventory

- `3bd6cd66` — clear chat
- `b14fce2c` — parts-list `j`/`k` navigation and explicit selection indicator
- `8b08d841` — slash command input in chat interface
- `4df096ce` — viewport display/lighting/grid improvements
- `f79cb340` — render command, viewport settings, parts display layout
- `1f43d0b6` — fix hidden chat focus trap, command alias cleanup, temporary review artifacts
- `b04d3f03` — faster rendering during move
- `0c30303c` — Raspberry Pi/Ghostty terminal fixes
- `e2bcf457` — faster Kitty graphics
- `9da71b35` — merge of RPi terminal fixes
- `322b05ee` — OpenRouter/Ollama support and chat slash autocomplete
- `60b34de3` — remove temporary docs and accidental PNG
- `9bd69c1b` — remove branch-local changelog entries

## Candidate PR 1: TUI chat slash commands and chat-history clearing

**Summary:** Make the chat panel double as a command line when input starts with `/`. Adds discoverable slash-command suggestions, Tab completion, Up/Down suggestion selection, command execution through `App::process_command`, and a `/clear` path that aborts in-flight chat and clears visible + persisted chat history.

**Relevant commits:** `3bd6cd66`, `8b08d841`, parts of `322b05ee`.

**Relevant files / lines:**
- `crates/vcad-cli/src/ui/chat.rs:32-330` — `SlashCommandSuggestion`, command list, suggestion scoring, completion helpers.
- `crates/vcad-cli/src/ui/chat.rs:350-453` — `slash_selected_index` state and `ChatPanel::clear()`.
- `crates/vcad-cli/src/ui/chat.rs:694-836` — render slash suggestion popup and improved input-line behavior.
- `crates/vcad-cli/src/ui/chat.rs:955-958` — completion preference test.
- `crates/vcad-cli/src/input.rs:693-719` — submit `/...` messages through `process_command` instead of chat API.
- `crates/vcad-cli/src/input.rs:720-771` — reset/advance slash selection and complete with Tab.
- `crates/vcad-cli/src/input.rs:979-990` — `/` focuses chat and seeds slash-command prefix.
- `crates/vcad-cli/src/app.rs:1356-1365` — `clear` command aborts chat, clears message/session state, and calls `ChatPanel::clear()`.
- `crates/vcad-cli/src/chat_session.rs:556` — chat-history clearing helper.

## Candidate PR 2: TUI render/screenshot command

**Summary:** Add a TUI command for rendering the current viewport to PNG, with optional path, dimensions, and auto-open behavior. Exposes the command through command metadata, toolbar, command processing, and localization.

**Relevant commit:** `f79cb340`.

**Relevant files / lines:**
- `crates/vcad-cli/src/app.rs:990-1015` — `render_png()` and best-effort open helper.
- `crates/vcad-cli/src/app.rs:1250-1285` — `render`/`screenshot`/`image`/`png` command parsing, default path, dimensions, `--no-open`.
- `crates/vcad-cli/src/input.rs:321-328` — toolbar click opens inline render path input.
- `crates/vcad-cli/src/ui/toolbar.rs:299-305` — Render subtool in Export toolbar.
- `crates/vcad-app/src/commands.rs:481-490` — static command registry entry.
- `crates/vcad-i18n/locales/en.json:42` — `cmd.render.label` string.

## Candidate PR 3: Parts-list keyboard navigation and explicit multi-select UX

**Summary:** Improve TUI feature tree usability: `j`/`k` moves focus, Space/Enter toggles focused part, checkboxes show selection, root IDs are displayed for command-mode selection, and selection changes mark the viewport dirty.

**Relevant commits:** `b14fce2c`, parts of `f79cb340`.

**Relevant files / lines:**
- `crates/vcad-cli/src/app.rs:140-153` — focused-part index and render-dirty state.
- `crates/vcad-cli/src/input.rs:1063-1123` — `j`/`k`, Tab, clear, Space/Enter selection, and updated WASD/PageUp/PageDown bindings.
- `crates/vcad-cli/src/input.rs:406-474` — mouse selection now records pre-change selection and marks `render_dirty`.
- `crates/vcad-cli/src/app.rs:1308-1342` — command-mode `select`, `select_all`/`all`, and `deselect`/`clear_selection` support with dirty marking.
- `crates/vcad-cli/src/ui/tree.rs:146-170` — focus marker, checkbox, root ID in part label.
- `crates/vcad-cli/src/ui/tree.rs:200-231` — wider/sidebar rect and hit testing that account for dynamic toolbar height.

## Candidate PR 4: TUI layout cleanup for toolbar/sidebar and hidden chat focus trap

**Summary:** Fix UI layering/layout issues: sidebar starts below the actual toolbar height, inline tool input keeps toolbar height stable, and toggling chat keeps `open` and `focused` synchronized so a hidden panel does not trap keyboard input.

**Relevant commits:** `f79cb340`, `1f43d0b6`.

**Relevant files / lines:**
- `crates/vcad-cli/src/ui/toolbar.rs:631-651` — toolbar height/rect helpers aware of inline tool input.
- `crates/vcad-cli/src/ui/mod.rs:64-79` — computes sidebar top from current toolbar rect.
- `crates/vcad-cli/src/input.rs:89-139` — hit testing uses toolbar-aware sidebar position.
- `crates/vcad-cli/src/ui/tree.rs:11-23` — sidebar draw call takes a `top_y`.
- `crates/vcad-cli/src/app.rs:1350-1355` — `toggle_chat` synchronizes `chat.open` and `chat.focused`.

## Candidate PR 5: Faster keyboard movement and dirty viewport rendering

**Summary:** Make keyboard nudges much faster and reduce document bloat. Consecutive root `Translate` nodes are coalesced, cached mesh vertices are shifted directly because tessellation/topology does not change, and the main render loop skips viewport rerendering unless the scene or camera is dirty.

**Relevant commit:** `b04d3f03`, with supporting dirty flags from nearby selection changes.

**Relevant files / lines:**
- `crates/vcad-cli/src/app.rs:152-153` and `crates/vcad-cli/src/app.rs:220-225` — `render_dirty` app state and initialization.
- `crates/vcad-cli/src/app.rs:842-947` — translate-chain coalescing and cached mesh vertex offsetting.
- `crates/vcad-cli/src/app.rs:1571-1625` — resize/camera/dirty checks drive viewport rerendering.
- `crates/vcad-cli/src/app.rs:1627-1645` — pixel-protocol output only refreshed when dirty.
- `crates/vcad-cli/src/input.rs:1116-1123` — corrected WASD plane semantics and PageUp/PageDown vertical movement.

## Candidate PR 6: Terminal pixel-protocol overlay stability and Ghostty fallback

**Summary:** Improve Kitty/Ghostty/iTerm/Sixel behavior by keeping a stable image ID, deleting only on resize, placing Kitty images below text, invalidating overlay cells when the pixel viewport refreshes, and defaulting Ghostty to half-block rendering unless the user explicitly opts into Kitty.

**Relevant commits:** `0c30303c`, parts of `e2bcf457`.

**Relevant files / lines:**
- `crates/termview/src/output.rs:19-21` — remember last Kitty image size.
- `crates/termview/src/output.rs:64-76` — stable image ID and resize-only delete.
- `crates/termview/src/protocols/kitty.rs:46-72` — optional delete before upload and Kitty z-layer comments.
- `crates/termview/src/protocols/kitty.rs:85-91` — first chunk includes `z=-1`, quiet mode, and optional compression flag.
- `crates/termview/src/terminal.rs:52-68` — explicit `TERMVIEW_PROTOCOL`/`VCAD_PROTOCOL` override handling.
- `crates/termview/src/terminal.rs:101-116` — Ghostty defaults to `HalfBlock` because full-screen Kitty image + text overlay is unreliable.
- `crates/vcad-cli/src/ui/buffer.rs:175-183` — begin each pixel-overlay frame from an empty text layer.
- `crates/vcad-cli/src/ui/buffer.rs:255-263` — invalidate previous cell state so overlays repaint over refreshed images.
- `crates/vcad-cli/src/app.rs:1627-1645` — output pixel image, invalidate text diff, then draw overlay.
- `crates/vcad-cli/src/ui/theme.rs:153-158` — pixel-protocol background treatment for overlays.

## Candidate PR 7: Faster Kitty payloads via zlib compression

**Summary:** Compress Kitty RGBA payloads with fast zlib when it reduces size, lowering terminal bandwidth for mostly-flat CAD previews. Provides `TERMVIEW_KITTY_COMPRESS=0` escape hatch.

**Relevant commit:** `e2bcf457`.

**Relevant files / lines:**
- `crates/termview/Cargo.toml:12` — adds `flate2` dependency.
- `Cargo.lock:1548` and `Cargo.lock:4034` — lockfile updates for `flate2` usage.
- `crates/termview/src/protocols/kitty.rs:9` — imports `flate2` encoder.
- `crates/termview/src/protocols/kitty.rs:74-91` — use compressed payload and advertise `o=z` to Kitty.
- `crates/termview/src/protocols/kitty.rs:144-158` — `kitty_payload()` compression helper and env var opt-out.

## Candidate PR 8: Terminal rasterizer visual improvements

**Summary:** Improve low-resolution terminal viewport readability with brighter CAD-style lighting, fill/head/rim contributions, silhouette/crease outlining, and a screen-space construction grid that no longer becomes thick world-space slabs at grazing angles.

**Relevant commits:** `4df096ce`, parts of `f79cb340`.

**Relevant files / lines:**
- `crates/termview/src/rasterize.rs:289-344` — key/fill/head/rim lighting and cooler highlights.
- `crates/termview/src/rasterize.rs:374-421` — in-place silhouette/crease outline pass from pick/depth buffers.
- `crates/termview/src/rasterize.rs:458-462` — brighter grid colors.
- `crates/termview/src/rasterize.rs:509-551` — convert grid quads into centerlines and draw in screen space.
- `crates/termview/src/rasterize.rs:554-610` — projection, pixel plotting, and constant-width grid line drawing.
- `crates/vcad-cli/src/app.rs:1028-1041` — updated object and selected-object colors for terminal rendering.

## Candidate PR 9: OpenRouter/Ollama/OpenAI-compatible chat backends

**Summary:** Generalize the app chat API beyond Anthropic. Adds environment-driven backend selection for Anthropic, OpenRouter, Ollama, OpenAI, and OpenAI-compatible servers; converts Anthropic tool/message shapes to OpenAI-compatible shapes; streams text/tool calls back through the existing SSE contract; forwards provider env vars in Vite dev API; and adds a helper script for running the TUI against OpenRouter + Kitty.

**Relevant commit:** `322b05ee`.

**Relevant files / lines:**
- `packages/app/api/chat.ts:34-37` — Anthropic model env overrides and OpenAI-compatible token cap.
- `packages/app/api/chat.ts:39-67` — `ChatBackend` and provider selection from env (`VCAD_CHAT_PROVIDER`, `OPENROUTER_*`, `OLLAMA_*`, `OPENAI_*`).
- `packages/app/api/chat.ts:391-468` — OpenAI-compatible message/tool conversion helpers.
- `packages/app/api/chat.ts:470-589` — OpenAI-compatible streaming parser for text/tool-call deltas and usage.
- `packages/app/api/chat.ts:856-864` — backend configuration error handling.
- `packages/app/api/chat.ts:939-943` — Anthropic-only safety classifier is skipped when unavailable for non-Anthropic backends.
- `packages/app/api/chat.ts:1001-1033` — branch between Anthropic Messages API and `/chat/completions` request.
- `packages/app/api/chat.ts:1171-1181` — stream through Anthropic or OpenAI-compatible pipe while preserving usage accounting.
- `packages/app/vite.config.ts:131-144` — dev API forwards provider/model env vars.
- `scripts/vcad-tui-openrouter-kitty.sh:1-18` — script purpose and env contract.
- `scripts/vcad-tui-openrouter-kitty.sh:48-56` — provider/model/protocol setup and key validation.
- `scripts/vcad-tui-openrouter-kitty.sh:84-105` — server readiness wait and TUI launch diagnostics.

## Cleanup-only changes, not standalone feature PRs

**Summary:** Remove temporary branch artifacts and branch-local changelog files. These should probably stay folded into whatever branch-prep work produces upstream PRs rather than become their own PR.

**Relevant commits:** `60b34de3`, `9bd69c1b`.

**Relevant paths:**
- Removed temporary docs: `BRANCH_FEATURE_FIX_REVIEW.md`, `cheatsheet.md`, `docs/research/vcad-cli-tui-ghostty-rpi5-findings.md`.
- Removed accidental binary: `vcad-render.png`.
- Removed branch-local changelog entries: `changelog/entries/2026-05-17-cli-tui-ghostty-pixel-fix.json`, `2026-05-18-cli-tui-render-command.json`, `2026-05-18-cli-tui-viewport-colors.json`, `2026-05-20-chat-openrouter-ollama.json`, `2026-05-20-cli-chat-slash-autocomplete.json`.

## Suggested PR ordering

1. Terminal rasterizer visual improvements (PR 8) — isolated to `termview` rendering.
2. Kitty/Ghostty pixel overlay stability (PR 6), then Kitty compression (PR 7) — related but reviewable separately.
3. Faster movement / render-dirty loop (PR 5) — likely benefits TUI users independent of UX changes.
4. Parts-list selection UX (PR 3) and layout/focus fixes (PR 4).
5. Render/screenshot command (PR 2).
6. Chat slash commands and clear chat (PR 1).
7. OpenRouter/Ollama backend support (PR 9) — largest API-facing change; easiest to review after TUI command changes are settled.
