# Wkeys
An on-screen keyboard for wayland desktop written by rust using GTK-rs.

**Notice: Wkeys requires your wayland wm support [Virtual keyboard](https://wayland.app/protocols/virtual-keyboard-unstable-v1) and [wlr layer shell](https://wayland.app/protocols/wlr-layer-shell-unstable-v1)**

![Default Layout](./assets/default.png)

## Installation
#### Cargo

```shell
cargo add wkeys
```

#### Build from Source
Install dependencies, taking Arch Linux as an example.
```shell
paru gtk4 gtk4-layer-shell
```
Build project.
```shell
git clone https://github.com/ptazithos/wkeys.git
cargo Build 
```

## Configuration
The config files are at `~/.config/wkeys/`. The layout file is named `layout.toml`. The style file is named `style.css`.


The default layout file is: 
```toml
layout = [
    [
        { top_legend = '~', bottom_legend = '`', scan_code = 41 },
        { top_legend = '!', bottom_legend = '1', scan_code = 2 },
        { top_legend = '@', bottom_legend = '2', scan_code = 3 },
        { top_legend = '#', bottom_legend = '3', scan_code = 4 },
        { top_legend = '$', bottom_legend = '4', scan_code = 5 },
        { top_legend = '%', bottom_legend = '5', scan_code = 6 },
        { top_legend = '^', bottom_legend = '6', scan_code = 7 },
        { top_legend = '&', bottom_legend = '7', scan_code = 8 },
        { top_legend = '*', bottom_legend = '8', scan_code = 9 },
        { top_legend = '(', bottom_legend = '9', scan_code = 10 },
        { top_legend = ')', bottom_legend = '0', scan_code = 11 },
        { top_legend = '_', bottom_legend = '-', scan_code = 12 },
        { top_legend = '+', bottom_legend = '=', scan_code = 13 },
        { top_legend = 'Backspace', scan_code = 14, width = 2 },
    ],
    [
        { top_legend = 'Tab', scan_code = 15, width = 1.5 },
        { top_legend = 'Q', scan_code = 16 },
        { top_legend = 'W', scan_code = 17 },
        { top_legend = 'E', scan_code = 18 },
        { top_legend = 'R', scan_code = 19 },
        { top_legend = 'T', scan_code = 20 },
        { top_legend = 'Y', scan_code = 21 },
        { top_legend = 'U', scan_code = 22 },
        { top_legend = 'I', scan_code = 23 },
        { top_legend = 'O', scan_code = 24 },
        { top_legend = 'P', scan_code = 25 },
        { top_legend = '{', bottom_legend = '[', scan_code = 26 },
        { top_legend = '}', bottom_legend = ']', scan_code = 27 },
        { top_legend = '|', bottom_legend = '\', scan_code = 43, width = 1.5 },
    ],
    [
        { top_legend = 'Caps Lock', scan_code = 58, width = 1.75 },
        { top_legend = 'A', scan_code = 30 },
        { top_legend = 'S', scan_code = 31 },
        { top_legend = 'D', scan_code = 32 },
        { top_legend = 'F', scan_code = 33 },
        { top_legend = 'G', scan_code = 34 },
        { top_legend = 'H', scan_code = 35 },
        { top_legend = 'J', scan_code = 36 },
        { top_legend = 'K', scan_code = 37 },
        { top_legend = 'L', scan_code = 38 },
        { top_legend = ':', bottom_legend = ';', scan_code = 39 },
        { top_legend = '"', bottom_legend = "'", scan_code = 40 },
        { top_legend = 'Enter', scan_code = 28, width = 2.25 },
    ],
    [
        { top_legend = 'Shift', scan_code = 42, width = 2.25 },
        { top_legend = 'Z', scan_code = 44 },
        { top_legend = 'X', scan_code = 45 },
        { top_legend = 'C', scan_code = 46 },
        { top_legend = 'V', scan_code = 47 },
        { top_legend = 'B', scan_code = 48 },
        { top_legend = 'N', scan_code = 49 },
        { top_legend = 'M', scan_code = 50 },
        { top_legend = '<', bottom_legend = ',', scan_code = 51 },
        { top_legend = '>', bottom_legend = '.', scan_code = 52 },
        { top_legend = '?', bottom_legend = '/', scan_code = 53 },
        { top_legend = 'Shift', scan_code = 54, width = 2.75 },
    ],
    [
        { top_legend = 'Ctrl', scan_code = 29, width = 1.25 },
        { top_legend = 'Win', scan_code = 91, width = 1.25 },
        { top_legend = 'Alt', scan_code = 56, width = 1.25 },
        { top_legend = 'Space', scan_code = 57, width = 6.25 },
        { top_legend = 'Alt', scan_code = 56, width = 1.25 },
        { top_legend = 'Win', scan_code = 92, width = 1.25 },
        { top_legend = 'Menu', scan_code = 93, width = 1.25 },
        { top_legend = 'Ctrl', scan_code = 29, width = 1.25 },
    ],
]

```
For more scan codes, check this: [linux/input-event-codes.h](https://github.com/torvalds/linux/blob/43fb83c17ba2d63dfb798f0be7453ed55ca3f9c2/include/uapi/linux/input-event-codes.h#L4)

The default style file is:

```css
window {
    background-color: rgba(0, 0, 0, 0);
}

button {
    background-color: rgb(220, 220, 220);
    background-image: none;
    border-radius: 0;
    margin: 1px;
    padding: 12px;

}

button>label {
    color: black;
}

button:hover {
    background-color: rgb(255, 255, 255);
}

button:active {
    background-color: rgb(255, 255, 255);
}

button:checked {
    background-color: rgb(255, 255, 255);
}
```

## IPC
Close Wkeys by IPC message
```shell
wkeys --message close
```

## License

This repository is licensed under the [MIT License](LICENSE).
