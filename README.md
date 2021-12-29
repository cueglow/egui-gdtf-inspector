# egui-gdtf-inspector

This is a low-quality reference application to drive development of the
[GDTF](https://gdtf-share.com/) library ecosystem in Rust. 

GDTF is a file format for the description of lighting fixtures in the
entertainment industry. 

:construction: **Status:** Very much under construction :construction:

## How To Build

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. On Linux, install the GTK3 development libraries (`libgtk-3-dev` on Debian/Ubuntu and `gtk3` on arch, these are required for the [file dialog](https://github.com/PolyMeilex/rfd))
3. Clone `gdtf_parser` to a sibling folder (local dependency for ease of development):
```sh
git clone https://github.com/michaelhugi/gdtf_parser.git
```
4. Clone and run
```sh
git clone https://github.com/cueglow/egui-gdtf-inspector.git
cd egui-gdtf-inspector
cargo run
```



