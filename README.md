## *30 seconds to launch node? not better 30 minutes to write it in rust?*

First argument path to the epub, second is path and name to json file, and the second for the epub cover. Set RUST_LOG=debug to get debug output

Its a very basic version, so stay calm and inform be of everything

### to compile:
setup cross ( docker etc ):
```
cargo install cross
```
build it:
```
~/.cargo/bin/cross build --release --target armv7-unknown-linux-musleabihf
```
