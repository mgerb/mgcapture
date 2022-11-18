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
- place shortcut to `mgcapture.exe` in this folder

### Linux

- install dependencies

```
sudo apt install libxcb1-dev libxrandr2 libdbus-1-dev

```

- copy `mgcapture` to `/home/user/Desktop/screenshots`
- create a run script `/home/user/Desktop/screenshots/run.sh`:

```
cd /home/user/Desktop/screenshots && ./mgcapture
```

- the following may vary depending on OS (tested on Pop!\_OS)
  - open "Startup Application Preferences"
  - add new command: `/home/user/Desktop/screenshots/run.sh`

## Compiling

Cross compile for both Linux and Windows (only tested on a Linux machine)

`make clean all`

## Credits

- [screenshots-rs](https://github.com/nashaofu/screenshots-rs)
- [serde](https://github.com/serde-rs/serde)
- [chrono](https://github.com/chronotope/chrono)
