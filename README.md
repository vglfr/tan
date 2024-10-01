## TAN - Tiny Annotation Tool

Simple vim-like tool for NER annotation.

15-second usage gif

Not just an annotation tool. General annotation tool.
Something which takes input text, keeps it unchanged,
and allows any number of overlays over it. Overlays
could be added, removed, extended, merged, and visualized.

#### Installation

- `nix shell vglfr/tan#default` with Nix
- `cargo install tan` with crates.io

#### Usage

`tan [OPTIONS] [NAME]`

Options:

- `-f FORMAT`, `--format FORMAT` [default: spacy] [possible values: raw, spacy, tan]

#### Modes

| mode | description |
| ---- | ------- |
| `normal` | text preview and navigation |
| `command` |  |
| `visual` | .. |
| `x` | .. |
| `x` | .. |
| `x` | .. |

#### Keybindings

`normal` mode:

| key | command |
| --- | ------- |
| `x` | .. |
| `x` | .. |
| `x` | .. |
| `x` | .. |
| `x` | .. |
| `x` | .. |
| `x` | .. |
| `x` | .. |

`normal` mode:

#### Commands

     command | description             
          -- | --                      
`q`, `quit`  | quit                    
`w`, `write` | write file at tan format

TUI consists of:
- rendering and scrolling of an underlying text
- moving cursor and selection
- rendering of annotations
- annotation register
- statistics, help, misc

Tasks:
+ display static text in TUI
+ open file from CLI arg and display it
+ add cursor to TUI (/ and show it in different color)
+ navigate cursor with hjkl
+ wrap around newline with hjkl
+ tag / untag selection with hotkey (t / u ?)
+ display tag information in the bottom
+ save tagged file to disk
+ vertical scroll
+ horizontal wrap/scroll
+ restore hl wrapping moves
+ restore jk wrapping moves
+ fix visual selection
+ active tag
+ fix modal resize
+ load spacy format
+ hide/unhide tags
+ proper command mode (:)
+ extra vertical movements (c-n,c-p)
+ clap
+ wbSE movement
+ status line / hints (active tag -- filename | tags under cursor | command -- cursor posiiton)

- fix wrap line display
- toggle wrap/unwrap mode
- fix blinking (too much redraw? even on non-redrawing hl moves)
- multiline tagging
- overlapping tags

- help screen
- save spacy format (for QA)
- error handling
- debug logging
- terminal resize
- use mouse to set cursor position
- use mouse for selection
- use mouse wheel for scrolling
- virtual column
- helix-like select (when moving with wb)
- docx export
