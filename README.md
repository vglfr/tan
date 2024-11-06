## tan - Tiny Annotation Tool

Simple vim-like tool for NER annotation.

![demo](https://drive.google.com/file/d/1XFzLmEMq1ymivEF2qN8QGNBpFeGIGjTn/view?usp=sharing)

#### Features

- quickly visualize NER annotations from Spacy
- add new annotations using vim-like interface
- modify and delete existing annotations

#### Installation

- `nix shell vglfr/tan#default` with Nix
- `cargo install tan-annotation-tool` with crates.io

#### Usage

`tan [OPTIONS] [NAME]`

Options:

- `-f FORMAT`, `--format FORMAT` [default: plain] [possible values: plain, spacy, tan]

#### Formats

For now only plain and Spacy formats are supported.
Spacy NER annotations could be exported like this:

```python
import json
import spacy

nlp = spacy.load("en_core_web_sm")


with open("data/test.txt") as f:
    doc = nlp(f.read())

with open("data/test.json", "w") as f:
    json.dump(doc.to_json(), f)
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
