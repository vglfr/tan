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
+ help screen
+ deprecate overflow mode
+ fix wrap line display
+ fix file reading
+ fix cursor movement
+ fix blinking (too much redraw? even on non-redrawing hl moves)
+ fix statusline redraw (lingering tags)
+ fix tag display across virtual rows
+ overlapping tags
+ multiline tags
+ drop mode indicator from statusline (display visual instead of tag in visual)
+ multiline untag
+ multiline visual
+ multiline tagging
+ fix modal render
+ fix status styling
+ Coralie demo
+ fix modal width & height
+ refactor color mode
+ handle resizing
+ error messages
+ finish refactoring
+ fix blinking
+ ascii video
+ github tags

- Nix flake
- Cargo package
- lists to add
- forums to publish

- overlapping untag
- terminal resize
- handle c-u c-w in command / name mode
- render line (visual hl move optimization)
- error handling
- use mouse to set cursor position
- use mouse for selection
- use mouse wheel for scrolling

- save spacy format (for QA)
- debug logging
- virtual column (try to keep column position after newline)
- docx export

- redux overflow mode
- toggle wrap/overflow mode
