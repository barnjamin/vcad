//! Right-docked chat sidebar — conversational AI + debug logs.
//!
//! Layout mirrors `packages/app/src/components/ChatSidebar.tsx`: a
//! full-height column on the right, anchored below the menu bar and above
//! the status bar. Messages stack top-to-bottom with the input line pinned
//! to the bottom.

use super::buffer::{set_char, CellBuffer, Rect};
use super::theme;

/// Maximum number of lines retained in scrollback.
const MAX_LINES: usize = 500;

/// Kind of chat line, determines styling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatLineKind {
    /// User message.
    User,
    /// AI assistant response.
    Assistant,
    /// Debug / system log.
    Debug,
}

/// A single line in the chat panel.
#[derive(Debug, Clone)]
pub struct ChatLine {
    pub text: String,
    pub kind: ChatLineKind,
}

/// Slash-command suggestion shown while typing in the chat input.
#[derive(Debug, Clone, Copy)]
pub struct SlashCommandSuggestion {
    pub name: &'static str,
    pub usage: &'static str,
    pub description: &'static str,
    aliases: &'static [&'static str],
}

const MAX_SLASH_SUGGESTIONS: usize = 5;

const SLASH_COMMANDS: &[SlashCommandSuggestion] = &[
    SlashCommandSuggestion {
        name: "cube",
        usage: "cube [size]",
        description: "Add cube",
        aliases: &["box"],
    },
    SlashCommandSuggestion {
        name: "cylinder",
        usage: "cylinder [radius] [height]",
        description: "Add cylinder",
        aliases: &["cyl", "tube"],
    },
    SlashCommandSuggestion {
        name: "sphere",
        usage: "sphere [radius]",
        description: "Add sphere",
        aliases: &["ball"],
    },
    SlashCommandSuggestion {
        name: "cone",
        usage: "cone [radius] [height]",
        description: "Add cone",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "select",
        usage: "select <id> [id...]",
        description: "Select parts",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "select_all",
        usage: "select_all",
        description: "Select all parts",
        aliases: &["all"],
    },
    SlashCommandSuggestion {
        name: "deselect",
        usage: "deselect",
        description: "Clear selection",
        aliases: &["clear_selection"],
    },
    SlashCommandSuggestion {
        name: "delete",
        usage: "delete",
        description: "Delete selection",
        aliases: &["del", "rm"],
    },
    SlashCommandSuggestion {
        name: "move",
        usage: "move <dx> <dy> <dz>",
        description: "Move selection",
        aliases: &["translate"],
    },
    SlashCommandSuggestion {
        name: "rotate",
        usage: "rotate <rx> <ry> <rz>",
        description: "Rotate selection in degrees",
        aliases: &["spin", "turn"],
    },
    SlashCommandSuggestion {
        name: "scale",
        usage: "scale [factor]",
        description: "Scale selection uniformly",
        aliases: &["resize"],
    },
    SlashCommandSuggestion {
        name: "union",
        usage: "union",
        description: "Union selected parts",
        aliases: &["combine"],
    },
    SlashCommandSuggestion {
        name: "difference",
        usage: "difference",
        description: "Subtract selected parts",
        aliases: &["subtract", "cut"],
    },
    SlashCommandSuggestion {
        name: "intersection",
        usage: "intersection",
        description: "Intersect selected parts",
        aliases: &["intersect"],
    },
    SlashCommandSuggestion {
        name: "fillet",
        usage: "fillet [radius]",
        description: "Fillet selection",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "chamfer",
        usage: "chamfer [distance]",
        description: "Chamfer selection",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "shell",
        usage: "shell [thickness]",
        description: "Shell selection",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "pattern",
        usage: "pattern [count]",
        description: "Pattern selection",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "mirror",
        usage: "mirror",
        description: "Mirror selection",
        aliases: &["flip"],
    },
    SlashCommandSuggestion {
        name: "save",
        usage: "save [path.vcad]",
        description: "Save document",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "export",
        usage: "export <path.stl>",
        description: "Export STL",
        aliases: &["export_stl"],
    },
    SlashCommandSuggestion {
        name: "render",
        usage: "render <path.png> [WIDTHxHEIGHT] [--no-open]",
        description: "Render PNG image",
        aliases: &["screenshot", "image", "png"],
    },
    SlashCommandSuggestion {
        name: "undo",
        usage: "undo",
        description: "Undo",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "redo",
        usage: "redo",
        description: "Redo",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "new",
        usage: "new",
        description: "New document",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "sketch",
        usage: "sketch",
        description: "Enter sketch mode",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "clear",
        usage: "clear",
        description: "Clear chat history",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "toggle_sidebar",
        usage: "toggle_sidebar",
        description: "Show/hide feature tree",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "toggle_chat",
        usage: "toggle_chat",
        description: "Show/hide chat",
        aliases: &[],
    },
    SlashCommandSuggestion {
        name: "camera_iso",
        usage: "camera_iso",
        description: "Isometric camera",
        aliases: &["iso"],
    },
    SlashCommandSuggestion {
        name: "camera_top",
        usage: "camera_top",
        description: "Top camera",
        aliases: &["top"],
    },
    SlashCommandSuggestion {
        name: "camera_front",
        usage: "camera_front",
        description: "Front camera",
        aliases: &["front"],
    },
    SlashCommandSuggestion {
        name: "camera_right",
        usage: "camera_right",
        description: "Right camera",
        aliases: &["right"],
    },
    SlashCommandSuggestion {
        name: "camera_fit",
        usage: "camera_fit",
        description: "Fit camera",
        aliases: &["fit"],
    },
    SlashCommandSuggestion {
        name: "cycle_theme",
        usage: "cycle_theme",
        description: "Cycle theme",
        aliases: &["theme"],
    },
    SlashCommandSuggestion {
        name: "quit",
        usage: "quit",
        description: "Quit",
        aliases: &["q"],
    },
];

/// Return slash-command suggestions for the current chat input.
pub fn slash_command_suggestions(input: &str) -> Vec<SlashCommandSuggestion> {
    let Some(rest) = input.strip_prefix('/') else {
        return Vec::new();
    };
    let query = rest
        .trim_start()
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_lowercase();
    if query.is_empty() {
        return SLASH_COMMANDS
            .iter()
            .copied()
            .take(MAX_SLASH_SUGGESTIONS)
            .collect();
    }

    let mut scored: Vec<(u8, usize, SlashCommandSuggestion)> = SLASH_COMMANDS
        .iter()
        .copied()
        .filter_map(|cmd| {
            let name = cmd.name;
            let description = cmd.description.to_lowercase();
            let alias_prefix = cmd.aliases.iter().any(|alias| alias.starts_with(&query));
            let alias_contains = cmd.aliases.iter().any(|alias| alias.contains(&query));

            let rank = if name == query {
                0
            } else if name.starts_with(&query) {
                // Prefer the canonical command name over aliases, so `/clea`
                // completes to `clear` instead of `deselect`'s
                // `clear_selection` alias.
                1
            } else if alias_prefix {
                2
            } else if name.contains(&query) {
                3
            } else if alias_contains {
                4
            } else if description.contains(&query) {
                5
            } else {
                return None;
            };
            Some((rank, name.len(), cmd))
        })
        .collect();

    scored.sort_by_key(|(rank, name_len, cmd)| (*rank, *name_len, cmd.name));
    scored
        .into_iter()
        .take(MAX_SLASH_SUGGESTIONS)
        .map(|(_, _, cmd)| cmd)
        .collect()
}

/// Completion text for Tab in the chat input.
pub fn slash_command_completion(input: &str) -> Option<&'static str> {
    slash_command_completion_at(input, 0)
}

/// Completion text for Tab using the currently highlighted suggestion.
pub fn slash_command_completion_at(input: &str, selected_index: usize) -> Option<&'static str> {
    let suggestions = slash_command_suggestions(input);
    suggestions
        .get(selected_index.min(suggestions.len().saturating_sub(1)))
        .map(|cmd| cmd.name)
}

/// Persistent chat sidebar state.
pub struct ChatPanel {
    /// Whether the sidebar is rendered.
    pub open: bool,
    /// Whether the input has keyboard focus — when true, key events route
    /// to the chat input instead of the viewport/menu.
    pub focused: bool,
    /// Current input buffer.
    pub input: String,
    /// Chat lines (messages + debug logs).
    pub lines: Vec<ChatLine>,
    /// Message history for Up/Down recall (user messages only).
    pub history: Vec<String>,
    /// Index into history for Up/Down navigation (`None` = editing new input).
    pub history_index: Option<usize>,
    /// Scroll offset (0 = bottom, increases upward).
    pub scroll: usize,
    /// Highlighted slash-command suggestion. Tab completes this row; Up/Down
    /// change it while slash suggestions are visible.
    pub slash_selected_index: usize,
    /// Saved input when navigating history.
    saved_input: String,
}

impl ChatPanel {
    /// Create a chat panel docked to the right. Open but not focused by
    /// default — so CAD hotkeys still work until the user explicitly
    /// focuses the input via `` ` `` / Tab / clicking the sidebar.
    pub fn new() -> Self {
        Self {
            open: true,
            focused: false,
            input: String::new(),
            lines: Vec::new(),
            history: Vec::new(),
            history_index: None,
            scroll: 0,
            slash_selected_index: 0,
            saved_input: String::new(),
        }
    }

    /// Push a line, capping at MAX_LINES.
    fn push_line(&mut self, text: String, kind: ChatLineKind) {
        self.lines.push(ChatLine { text, kind });
        if self.lines.len() > MAX_LINES {
            self.lines.remove(0);
        }
    }

    /// Log a debug/system message.
    pub fn debug(&mut self, msg: impl Into<String>) {
        self.push_line(msg.into(), ChatLineKind::Debug);
    }

    /// Navigate history up.
    pub fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }
        match self.history_index {
            None => {
                self.saved_input = self.input.clone();
                let idx = self.history.len() - 1;
                self.history_index = Some(idx);
                self.input = self.history[idx].clone();
            }
            Some(idx) if idx > 0 => {
                let new_idx = idx - 1;
                self.history_index = Some(new_idx);
                self.input = self.history[new_idx].clone();
            }
            _ => {}
        }
    }

    /// Navigate history down.
    pub fn history_down(&mut self) {
        if let Some(idx) = self.history_index {
            if idx + 1 < self.history.len() {
                let new_idx = idx + 1;
                self.history_index = Some(new_idx);
                self.input = self.history[new_idx].clone();
            } else {
                self.history_index = None;
                self.input = self.saved_input.clone();
            }
        }
    }

    /// Submit the current input as a user message.
    /// Returns the message text (for the caller to act on).
    pub fn send_message(&mut self) -> Option<String> {
        let msg = self.input.trim().to_string();
        if msg.is_empty() {
            return None;
        }
        self.push_line(msg.clone(), ChatLineKind::User);
        self.history.push(msg.clone());
        self.history_index = None;
        self.saved_input.clear();
        self.input.clear();
        self.scroll = 0;
        self.slash_selected_index = 0;
        Some(msg)
    }

    /// Add an assistant response line.
    pub fn assistant(&mut self, msg: impl Into<String>) {
        self.push_line(msg.into(), ChatLineKind::Assistant);
    }

    /// Clear visible chat and input/history state.
    pub fn clear(&mut self) {
        self.lines.clear();
        self.history.clear();
        self.history_index = None;
        self.input.clear();
        self.saved_input.clear();
        self.scroll = 0;
        self.slash_selected_index = 0;
    }
}

/// Desired sidebar width in cells. Clamped to 60% of the area width so the
/// viewport still has room on narrow terminals.
const SIDEBAR_WIDTH: u16 = 50;

/// Rows reserved at the top for the menu bar (1) + tool strip (up to 2).
/// The chat sidebar starts immediately below this and the tool strip is
/// rendered across the full width so its right edge is visually occluded
/// by the chat, matching ChatSidebar.tsx's layout under Header.tsx.
const TOP_OFFSET: u16 = 3;
/// Rows reserved at the bottom for the status bar.
const BOTTOM_OFFSET: u16 = 1;

/// Compute the right-docked sidebar rect. Spans from the row below the tool
/// strip down to the row above the status bar.
pub fn chat_rect(area: Rect) -> Rect {
    let max_width = (area.width * 3 / 5).max(20);
    let width = SIDEBAR_WIDTH.min(max_width);
    let x = area.x + area.width.saturating_sub(width);
    let y = area.y + TOP_OFFSET;
    let height = area.height.saturating_sub(TOP_OFFSET + BOTTOM_OFFSET);
    Rect::new(x, y, width, height)
}

/// Draw the chat panel overlay. `in_flight` controls whether the
/// streaming spinner renders in the title bar.
pub fn draw_chat(buf: &mut CellBuffer, panel: &ChatPanel, in_flight: bool, area: Rect) {
    let rect = chat_rect(area);
    if rect.height < 4 || rect.width < 10 {
        return;
    }

    let top = rect.y;
    let bot = rect.y + rect.height - 1;
    let left = rect.x;
    let right = rect.x + rect.width - 1;

    // Fill background
    for y in rect.y..rect.y + rect.height {
        for x in rect.x..rect.x + rect.width {
            set_char(buf, x, y, ' ', theme::SURFACE(), theme::SURFACE());
        }
    }

    // Rounded border
    set_char(
        buf,
        left,
        top,
        '\u{250C}',
        theme::BORDER(),
        theme::SURFACE(),
    );
    set_char(
        buf,
        right,
        top,
        '\u{2510}',
        theme::BORDER(),
        theme::SURFACE(),
    );
    set_char(
        buf,
        left,
        bot,
        '\u{2514}',
        theme::BORDER(),
        theme::SURFACE(),
    );
    set_char(
        buf,
        right,
        bot,
        '\u{2518}',
        theme::BORDER(),
        theme::SURFACE(),
    );

    for x in (left + 1)..right {
        set_char(buf, x, top, '\u{2500}', theme::BORDER(), theme::SURFACE());
        set_char(buf, x, bot, '\u{2500}', theme::BORDER(), theme::SURFACE());
    }
    for y in (top + 1)..bot {
        set_char(buf, left, y, '\u{2502}', theme::BORDER(), theme::SURFACE());
        set_char(buf, right, y, '\u{2502}', theme::BORDER(), theme::SURFACE());
    }

    // Title in top border
    let title = " Chat ";
    let title_x = left + 2;
    for (i, ch) in title.chars().enumerate() {
        let x = title_x + i as u16;
        if x < right {
            set_char(buf, x, top, ch, theme::ACCENT(), theme::SURFACE());
        }
    }

    // Streaming spinner: when a request is in flight, animate a single
    // cell just after the title so the user can tell something is
    // actually happening between "send" and the first text token.
    if in_flight {
        let spinner_x = title_x + title.chars().count() as u16 + 1;
        if spinner_x < right {
            set_char(
                buf,
                spinner_x,
                top,
                spinner_frame(),
                theme::ACCENT(),
                theme::SURFACE(),
            );
        }
    }

    // Separator above input line
    let input_sep_y = bot - 1;
    let input_y = bot;

    if input_sep_y > top + 1 {
        for x in (left + 1)..right {
            set_char(
                buf,
                x,
                input_sep_y,
                '\u{2500}',
                theme::BORDER(),
                theme::SURFACE(),
            );
        }
        set_char(
            buf,
            left,
            input_sep_y,
            '\u{251C}',
            theme::BORDER(),
            theme::SURFACE(),
        );
        set_char(
            buf,
            right,
            input_sep_y,
            '\u{2524}',
            theme::BORDER(),
            theme::SURFACE(),
        );
    }

    // Output area: from top+1 to input_sep_y
    let output_top = top + 1;
    let output_bot = if input_sep_y > top + 1 {
        input_sep_y
    } else {
        output_top
    };
    let visible_rows = (output_bot - output_top) as usize;

    if visible_rows > 0 && !panel.lines.is_empty() {
        // Fixed two-cell prefix column ("▶ ", "✦ ", "│ ") — text wraps
        // at inner_w - 2 so continuation rows can indent past it.
        let inner_w = (right - left - 1) as usize;
        let text_w = inner_w.saturating_sub(2);

        // Flatten source lines into visual rows. `is_head` marks the first
        // row of a source line so the prefix icon only appears once; later
        // rows render spaces at the same column.
        let mut visual: Vec<(ChatLineKind, String, bool)> = Vec::new();
        for line in &panel.lines {
            let wrapped = wrap_text(&line.text, text_w);
            for (i, chunk) in wrapped.into_iter().enumerate() {
                visual.push((line.kind, chunk, i == 0));
            }
        }

        let total = visual.len();
        let scroll = panel.scroll.min(total.saturating_sub(visible_rows));
        let start = total.saturating_sub(visible_rows + scroll);
        let end = total.saturating_sub(scroll);

        for (i, (kind, text, is_head)) in visual[start..end].iter().enumerate() {
            let y = output_top + i as u16;
            if y >= output_bot {
                break;
            }

            let (prefix, prefix_fg, text_fg) = match kind {
                ChatLineKind::User => ("\u{25B6} ", theme::GREEN(), theme::TEXT()),
                ChatLineKind::Assistant => ("\u{2726} ", theme::ACCENT(), theme::PURPLE()),
                ChatLineKind::Debug => ("\u{2502} ", theme::BORDER(), theme::TEXT_MUTED()),
            };

            let mut cx = left + 1;

            // Prefix on the head row, two blank cells on continuation rows
            // so wrapped text stays visually aligned under the same column.
            if *is_head {
                for ch in prefix.chars() {
                    if (cx - left) as usize >= inner_w {
                        break;
                    }
                    set_char(buf, cx, y, ch, prefix_fg, theme::SURFACE());
                    cx += 1;
                }
            } else {
                for _ in 0..2 {
                    if (cx - left) as usize >= inner_w {
                        break;
                    }
                    set_char(buf, cx, y, ' ', theme::SURFACE(), theme::SURFACE());
                    cx += 1;
                }
            }

            for ch in text.chars() {
                if (cx - left) as usize >= inner_w {
                    break;
                }
                set_char(buf, cx, y, ch, text_fg, theme::SURFACE());
                cx += 1;
            }
        }

        // Scroll indicator — shows how many visual rows are hidden above.
        if scroll > 0 {
            let indicator = format!("[+{}]", scroll);
            let ix = right.saturating_sub(indicator.len() as u16 + 1);
            for (j, ch) in indicator.chars().enumerate() {
                set_char(
                    buf,
                    ix + j as u16,
                    output_top,
                    ch,
                    theme::TEXT_MUTED(),
                    theme::SURFACE(),
                );
            }
        }
    }

    draw_slash_command_suggestions(buf, panel, left, right, top, input_sep_y);

    // Input line on bottom border row
    for x in (left + 1)..right {
        set_char(buf, x, input_y, ' ', theme::SURFACE(), theme::SURFACE());
    }
    set_char(
        buf,
        left,
        input_y,
        '\u{2514}',
        theme::BORDER(),
        theme::SURFACE(),
    );
    set_char(
        buf,
        right,
        input_y,
        '\u{2518}',
        theme::BORDER(),
        theme::SURFACE(),
    );

    // Prompt
    let inner_left = left + 1;
    let inner_right = right;
    set_char(
        buf,
        inner_left,
        input_y,
        '>',
        theme::ACCENT(),
        theme::SURFACE(),
    );
    set_char(
        buf,
        inner_left + 1,
        input_y,
        ' ',
        theme::SURFACE(),
        theme::SURFACE(),
    );

    // Input text. Keep the cursor/end visible for long commands instead of
    // letting new text disappear beyond the right edge.
    let max_input = (inner_right - inner_left - 2) as usize;
    let input_chars: Vec<char> = panel.input.chars().collect();
    let input_len = input_chars.len();
    let skip = input_len.saturating_sub(max_input);
    for (i, ch) in input_chars.iter().skip(skip).take(max_input).enumerate() {
        set_char(
            buf,
            inner_left + 2 + i as u16,
            input_y,
            *ch,
            theme::TEXT(),
            theme::SURFACE(),
        );
    }

    // Cursor
    let cursor_x = inner_left + 2 + input_len.min(max_input) as u16;
    if cursor_x < inner_right {
        set_char(
            buf,
            cursor_x,
            input_y,
            '\u{2588}',
            theme::ACCENT(),
            theme::SURFACE(),
        );
    }

    // Hint
    let hint = "/cmd  Tab complete  Esc close";
    let hint_x = inner_right.saturating_sub(hint.len() as u16 + 1);
    if hint_x > cursor_x + 2 {
        for (j, ch) in hint.chars().enumerate() {
            set_char(
                buf,
                hint_x + j as u16,
                input_y,
                ch,
                theme::TEXT_MUTED(),
                theme::SURFACE(),
            );
        }
    }
}

fn draw_slash_command_suggestions(
    buf: &mut CellBuffer,
    panel: &ChatPanel,
    left: u16,
    right: u16,
    top: u16,
    input_sep_y: u16,
) {
    let suggestions = slash_command_suggestions(&panel.input);
    if suggestions.is_empty() || input_sep_y <= top + 2 {
        return;
    }

    let max_rows = suggestions.len().min(5);
    let start_y = input_sep_y.saturating_sub(max_rows as u16);
    let inner_left = left + 1;
    let inner_right = right;

    let selected = panel.slash_selected_index.min(max_rows.saturating_sub(1));
    for (row, suggestion) in suggestions.iter().take(max_rows).enumerate() {
        let y = start_y + row as u16;
        if y <= top || y >= input_sep_y {
            continue;
        }

        let is_selected = row == selected;
        let bg = if is_selected {
            theme::SELECTION_BG()
        } else {
            theme::CARD()
        };
        let usage_fg = if is_selected {
            theme::ACCENT()
        } else {
            theme::TEXT()
        };

        for x in (left + 1)..right {
            set_char(buf, x, y, ' ', bg, bg);
        }

        let marker = if is_selected { '›' } else { ' ' };
        set_char(buf, inner_left, y, marker, theme::ACCENT(), bg);

        let usage = format!("/{}", suggestion.usage);
        let mut cx = inner_left + 1;
        for ch in usage.chars() {
            if cx >= inner_right {
                break;
            }
            set_char(buf, cx, y, ch, usage_fg, bg);
            cx += 1;
        }

        let desc_x = inner_right.saturating_sub(suggestion.description.len() as u16 + 1);
        if desc_x > cx + 1 {
            for (i, ch) in suggestion.description.chars().enumerate() {
                set_char(buf, desc_x + i as u16, y, ch, theme::TEXT_MUTED(), bg);
            }
        }
    }
}

/// Pick one frame of the streaming spinner from the wall clock. Ten-frame
/// braille cycle matching Claude Code / oh-my-zsh style spinners, advancing
/// every 80 ms so the animation is visible but not frantic.
fn spinner_frame() -> char {
    const FRAMES: &[char] = &[
        '\u{280B}', '\u{2819}', '\u{2839}', '\u{2838}', '\u{283C}', '\u{2834}', '\u{2826}',
        '\u{2827}', '\u{2807}', '\u{280F}',
    ];
    let ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    FRAMES[((ms / 80) as usize) % FRAMES.len()]
}

/// Word-wrap `text` to lines no wider than `width` cells. Soft-breaks on
/// whitespace; if a single word is longer than `width` it's hard-broken
/// across as many rows as needed. An empty input still produces a single
/// empty row so scroll math stays consistent.
fn wrap_text(text: &str, width: usize) -> Vec<String> {
    if width == 0 {
        return vec![text.to_string()];
    }
    if text.is_empty() {
        return vec![String::new()];
    }

    let mut out: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut current_len = 0usize;

    let push_hard_break = |out: &mut Vec<String>, word: &str, width: usize| {
        let mut remaining = word;
        while !remaining.is_empty() {
            let take: String = remaining.chars().take(width).collect();
            let take_bytes = take.len();
            out.push(take);
            remaining = &remaining[take_bytes..];
        }
    };

    for word in text.split_whitespace() {
        let word_len = word.chars().count();
        if current_len == 0 {
            if word_len > width {
                push_hard_break(&mut out, word, width);
            } else {
                current.push_str(word);
                current_len = word_len;
            }
        } else if current_len + 1 + word_len <= width {
            current.push(' ');
            current.push_str(word);
            current_len += 1 + word_len;
        } else {
            out.push(std::mem::take(&mut current));
            current_len = 0;
            if word_len > width {
                push_hard_break(&mut out, word, width);
            } else {
                current.push_str(word);
                current_len = word_len;
            }
        }
    }

    if !current.is_empty() {
        out.push(current);
    }
    if out.is_empty() {
        out.push(String::new());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wraps_long_lines_at_word_boundaries() {
        let wrapped = wrap_text("the quick brown fox jumps over the lazy dog", 15);
        // Each row should be ≤ 15 and split on whitespace.
        for row in &wrapped {
            assert!(row.chars().count() <= 15, "row too long: {row:?}");
        }
        assert!(wrapped.len() >= 2);
    }

    #[test]
    fn hard_breaks_oversize_words() {
        let wrapped = wrap_text("supercalifragilisticexpialidocious", 10);
        assert!(wrapped.iter().all(|r| r.chars().count() <= 10));
        // Should fully account for every char.
        let joined: String = wrapped.join("");
        assert_eq!(joined, "supercalifragilisticexpialidocious");
    }

    #[test]
    fn empty_input_returns_one_empty_row() {
        assert_eq!(wrap_text("", 20), vec![String::new()]);
    }

    #[test]
    fn zero_width_returns_full_text() {
        assert_eq!(wrap_text("anything", 0), vec!["anything".to_string()]);
    }

    #[test]
    fn slash_completion_prefers_canonical_name_prefix_over_alias() {
        assert_eq!(slash_command_completion("/clea"), Some("clear"));
    }
}
