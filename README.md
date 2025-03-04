# Wkeys
An on-screen keyboard for wayland desktop and a corresponding Cosmic applet written by rust using gtk-rs and libcosmic.

[wkeys](./wkeys/README.md) | [cosmic applet](./cosmic-applet/README.md)


![Screenshot](./assets/wkeys.gif)

## Installation
#### Install from AUR 
```shell
paru wkeys-git
```
#### Build from Source
```shell
git clone https://github.com/ptazithos/wkeys
cd wkeys
cargo build --release
```

## Notice
- Wkeys requires your wayland wm support [Virtual keyboard](https://wayland.app/protocols/virtual-keyboard-unstable-v1) and [wlr layer shell](https://wayland.app/protocols/wlr-layer-shell-unstable-v1)
- Cosmic applet only works with [Cosmic](https://github.com/pop-os/cosmic-epoch)



## License

This repository is licensed under the [MIT License](LICENSE).
