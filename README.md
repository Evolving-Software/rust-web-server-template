


# Rust Web Server Template
[![Web Server Status](https://github.com/Evolving-Software/rust-web-server-template/actions/workflows/general.yml/badge.svg)](https://github.com/Evolving-Software/rust-web-server-template/actions/workflows/general.yml)  [![Latest Commit Security Status](https://github.com/Evolving-Software/rust-web-server-template/actions/workflows/audit-on-push.yml/badge.svg)](https://github.com/Evolving-Software/rust-web-server-template/actions/workflows/audit-on-push.yml) [![Nightly Security Audit Status](https://github.com/Evolving-Software/rust-web-server-template/actions/workflows/scheduled-workflow.yml/badge.svg)](https://github.com/Evolving-Software/rust-web-server-template/actions/workflows/scheduled-workflow.yml)





This is a template for a web server written in Rust. It's intended to be a starting point for a web server written in Rust. It's not intended to be a production-ready web server, but it's a good starting point for one.

## Features

* [x] Logging
* [x] Configuration
* [x] Static file serving
* [x] Templating
* [x] JSON
* [x] Error handling
* [x] Database
* [x] Sessions
* [x] Authentication
* [x] Tests

## Usage

### Pre-Requsites
#### Windows
```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
````

To use this template, first install [Rust](https://www.rust-lang.org/). Then:

    $ git clone
    $ cd rust-web-server-template
    $ cargo build

To run the server:

    $ cargo run

To run the tests:

    $ cargo test

## License

This project is licensed under AGPLv3 or later. See the LICENSE file for details.