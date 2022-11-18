# MGCapture

Lightweight application to capture screenshots on interval. Runs in the background.

I made this with the intention of documenting some of the things I do on a daily basis.
Please do not use this maliciously.

## Config

A `config.json` file will be generated on startup with the following:

```
  {
    "output_directory": "screenshots",
    "interval_seconds": 600,
    "folder_format": "%Y-%m-%d",
    "file_format": "%Y-%m-%d_%H-%M-%S"
  }
```

File/folder format options listed [here.](https://docs.rs/chrono/latest/chrono/format/strftime/)

## Running on system startup

### Windows

- start
- run
- type in `shell:startup`
- place shortcut to mgcapture.exe in this folder

### Linux

- install the following packages: `sudo apt install libxcb1-dev libxrandr2 libdbus-1-dev`
- copy `mgcapture` to `$HOME/Desktop/screenshots` (or wherever)
- `crontab -e`
- add the line: `@reboot cd $HOME/Desktop/screenshots && ./mgcapture`

## Compiling

Cross compile for both Linux and Windows (only tested on a Linux machine)

`make clean all`

## Credits

- [screenshots-rs](https://github.com/nashaofu/screenshots-rs)
- [serde](https://github.com/serde-rs/serde)
- [chrono](https://github.com/chronotope/chrono)
