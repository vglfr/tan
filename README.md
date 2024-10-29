## tan - Tiny Annotation Tool

Simple vim-like tool for NER annotation.

15-second usage gif

Not just an annotation tool. General annotation tool.
Something which takes input text, keeps it unchanged,
and allows any number of overlays over it. Overlays
could be added, removed, extended, merged, and visualized.

#### Installation

- `nix shell vglfr/tan#default` with Nix
- `cargo install tan-annotation-tool` with crates.io

#### Usage

`tan [OPTIONS] [NAME]`

Options:

- `-f FORMAT`, `--format FORMAT` [default: raw] [possible values: raw, spacy, tan]

#### Formats

For now only SpaCy format is parsed.

```python
import json
import spacy

nlp = spacy.load("en_core_web_sm")
doc = nlp("data/test.txt")

with open("data/test.json", "wb") as f:
    f.write(doc.to_json())
```

#### Modes

| mode | description |
| -- | -- |
| `normal` | text preview and navigation |
| `visual` | visual selection |
| `command` | command prompt |
| `tag` | tag modal |

#### Keybindings

###### Modes

| key | command |
| -- | -- |
| `:` | command mode |
| `m` | tag mode |
| `v` | visual mode |

###### Cursor movements

| key | command |
| -- | -- |
| `h` | left |
| `j` | down |
| `k` | up |
| `l` | right |
| `H` | top of the screen |
| `M` | middle of the screen |
| `L` | bottom of the screen |
| `C-n` | screen down |
| `C-p` | screen up |
| `s` | start of the line |
| `e` | end of the line |
| `S` | start of the file |
| `E` | end of the file |
| `w` | word ahead |
| `b` | word behind |

###### Normal mode

| key | command |
| -- | -- |
| `t` | tag selection with active label |
| `u` | untag selection |

###### Tag mode

| key | command |
| -- | -- |
| `j` | next tag |
| `k` | previous tag |
| `h` | next color |
| `l` | previous color |
| `a` | add tag |
| `d` | delete tag |
| `i` | edit tag name |
| `v` | toggle active tag visibility |
| `V` | toggle all tag visibility |
| `Return` | rename tag / activate tag |

#### Commands

| command | description |
| -- | -- |
| `q`, `quit` | quit |
| `w`, `write` | write file at tan format |
