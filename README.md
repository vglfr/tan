## TAN - Tiny Annotation Tool

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
- use mouse to set cursor position
- scroll long text with jk
- use mouse wheel for scrolling
- select with wb
- use mouse for selection
+ tag / untag selection with hotkey (t / u ?)
- display tag information in the bottom
~ save tagged file to disk
