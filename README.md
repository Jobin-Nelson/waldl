# Waldl

A basic wallpaper downloader written in rust, uses wallhaven API for downloading wallpapers in original format. 

Download is asynchronous downloads the images into `$HOME/Pictures/Wallpapers/$(date "+%b_%d")/`
eg: `/home/jobin/Pictures/Wallpapers/aug_15/`

![waldl image](./images/waldl_image.png)

## Install

1. Clone the repo to anywhere in your system
    - `git clone --depth https://github.com/Jobin-Nelson/waldl.git`

2. cd into the waldl, build the binary
    - `cargo build --release`

3. Move the binary to somewhere in your `$PATH` variable, for me it is `$HOME/script/`
    - `mv target/release/waldl ~/script/`


## Dependencies

- **reqwest**
- **tokio**
- **serde**
- **futures**

I've also build the same thing in python you can view that [here](https://github.com/Jobin-Nelson/.dotfiles/blob/main/scripts/waldl.py).