# rs-get

A wget clone written in Rust

```sh
$ git clone git@github.com:0x-kys/rs-get && cd rs-get 
```

```sh
$ cargo build --release && mv ./target/release/rs-get /usr/bin
```

```sh
$ rs-get --help
wget clone w rusty

Usage: rs-get [OPTIONS] -u <url>

Options:
  -u <url>       URL to download
  -q             screamn't when getting stuff
  -h, --help     Print help
  -V, --version  Print version
```

