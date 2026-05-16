# vcad TUI cheat sheet

## Build / run from repo

```bash
# Build
cargo build -p vcad-cli

# Run TUI from source checkout
cargo run -p vcad-cli -- tui

# Run TUI with a file
cargo run -p vcad-cli -- tui path/to/file.vcad
```

## Install local patched `vcad`

```bash
cargo install --path crates/vcad-cli --force
```

Then run:

```bash
vcad tui
vcad tui path/to/file.vcad
```

---

## Startup

If the welcome screen is visible:

| Key | Action |
|---|---|
| `j` / `Down` | next item |
| `k` / `Up` | previous item |
| `Enter` | choose |
| `Esc` / `q` | dismiss |

---

## Normal mode

| Key | Action |
|---|---|
| `q` | quit |
| `:` or `/` | command palette / command input |
| `` ` `` | open + focus chat |
| `F6` | toggle chat |
| `Shift+S` | enter sketch mode |
| `1` | Create tab |
| `2` | Transform tab |
| `3` | Combine tab |
| `4` | Modify tab |
| `5` | Assembly tab |
| `6` | Simulate tab |
| `7` | Export tab |
| `Tab` | cycle/select parts |
| `j` / `k` | move part-list focus without changing selection |
| `Enter` / `Space` | toggle focused part selection |
| `Shift`/`Ctrl`/`Alt` + click | toggle part selection with mouse |
| `Esc` | clear selection |
| `x` / `Delete` / `Backspace` | delete selected |
| `u` | undo |
| `r` | redo |
| `Ctrl+S` | save |
| `Shift+R` | toggle ray tracing |
| `t` | cycle theme |

---

## Camera

| Key | Action |
|---|---|
| `Left` / `Right` | rotate camera horizontally |
| `Up` / `Down` | rotate camera vertically |
| `+` / `=` | zoom in |
| `-` | zoom out |
| mouse wheel | zoom |
| right-drag | orbit |
| middle-drag | pan |
| double-click viewport | reset camera |

---

## Move selected part

After selecting a part:

| Key | Action |
|---|---|
| `w` | move +Z |
| `s` | move -Z |
| `a` | move -X |
| `d` | move +X |

For precise moves, use command mode:

```text
:move 10 0 0
:move 0 0 5
```

---

## Rotate / scale selected part

Use command mode:

```text
:rotate 0 15 0
:rotate 90 0 0
:rotate 0 0 -45
:scale 2
```

Format:

```text
rotate <x-degrees> <y-degrees> <z-degrees>
```

No parentheses/commas currently.

---

## Command mode

Open with `:` or `/`.

| Key | Action |
|---|---|
| type | filter commands or enter raw command |
| `Up` / `Down` | select palette item |
| `Enter` | execute |
| `Esc` | cancel |
| `Backspace` | delete character |

Useful commands:

```text
cube
cylinder
sphere
cone

move 5 0 0
rotate 0 15 0
scale 2

select 1 2
select_all
union
difference
intersection

fillet 2
chamfer 2
shell 1
pattern 3
mirror

save
export output.stl

select_all
deselect
delete
undo
redo
new

toggle_sidebar
toggle_chat
clear_chat

camera_iso
camera_top
camera_front
camera_right
camera_fit

sketch
```

---

## Chat sidebar

| Key | Action |
|---|---|
| `` ` `` | open/focus chat, or close when focused |
| `F6` | toggle chat |
| `Enter` | send message |
| `Esc` | close/unfocus chat |
| `Up` / `Down` | recall message history |
| `Shift+Up` / `Shift+Down` | scroll chat |
| `PageUp` / `PageDown` | scroll chat |

Clear chat:

```text
:clear_chat
```

or:

```text
/chat_clear
```

If chat is focused, command keys go into chat input. Press `Esc` first.

---

## Menu bar

| Key | Action |
|---|---|
| `Alt+F` | File |
| `Alt+E` | Edit |
| `Alt+V` | View |
| `Alt+T` | Tools |
| `Alt+H` | Help |
| arrows | navigate menu |
| `Enter` | run focused menu item |
| `Esc` | close menu |

---

## Sketch mode

Enter with:

```text
:sketch
```

or `Shift+S`.

| Key | Action |
|---|---|
| `l` | line tool |
| `r` | rectangle tool |
| `c` | circle tool |
| `Esc` | exit sketch mode |
