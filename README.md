## TAN - Tiny Annotation Tool

One phrase pitch.

15-second usage gif

One paragraph detalization.

#### Installation

- nix flake
- crates.io

#### Command line options

#### Keybindings

Not just an annotation tool. General annotation tool.
Something which takes input text, keeps it unchanged,
and allows any number of overlays over it. Overlays
could be added, removed, extended, merged, and visualized.
Annotation results could be stored in a centralized database (bad)
or in the same file (thus creating a docx-like container).
Second option is certainly more appealing. It better be
a custom line-based rich format (.tan) with a possibility
for docx export.

Each annotation has an id, bounds (exactly two of them,
lower and upper, which could span several lines), label, and color.
Annotations could overlap.

There is a centralized register of all available annotations
with a CRUD over them.

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

~ wbSE movement
- status line / hints (active tag -- filename | tags under cursor | command -- cursor posiiton)
- fix wrap line display
- toggle wrap/unwrap mode
- fix blinking (too much redraw? even on non-redrawing hl moves)
- save spacy format (for QA)

- overlapping tags
- virtual column
- error handling
- debug logging
- clap
- display tag hotkeys in modal
- multiline tagging
- helix-like select (when moving with wb)
- terminal resize
- help screen
- use mouse to set cursor position
- use mouse for selection
- use mouse wheel for scrolling
