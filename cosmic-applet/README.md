# Cosmic Applet Wkeys
A cosmic applet for wkeys written by rust using libcosmic

![Screenshot](../assets/wkeys.gif)

## Build from Source
```shell
git clone https://github.com/ptazithos/wkeys.git
cd cosmic-applet
cargo Build 
```

## Installation
### Manually:
Copy the applet to `/usr/bin`
```shell
sudo cp ./target/release/cosmic-applet-wkeys /usr/bin
```
Copy the .desktop file to `usr/share/applications`
```shell
sudo cp ./cosmic-applet/assets/wkeys-applet.desktop /usr/share/applications/
```
Add `wkeys` applet in `Cosmic Settings/Desktop/{Dock | Panel}`
