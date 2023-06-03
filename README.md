<h1 align="center">
    X-Launcher 🚀
</h1>
<h4 align="center">
    A cross-platform tool-agnostic project automation launcher.
</h4>
<p align="center">
  <a href="https://github.com/silitics/x-launcher#licensing"><img alt="License: MIT OR Apache 2.0" src="https://img.shields.io/crates/l/x-launcher"></a>
  <a href="https://crates.io/crates/x-launcher"><img alt="X-Launcher Rust Crate" src="https://img.shields.io/crates/v/x-launcher?label=crates.io"></a>  
</p>
<p align="center">
  🚧 X-Launcher is still experimental and has not been thoroughly tested. 🚧
</p>

💡 **TL;DR**: X-Launcher is a simple binary `x` which, when executed, scans for an `xfile` starting in the invocation directory and working its way up. When it finds an `xfile`, it uses it to launch project-specific tooling either (a) by executing the `xfile` itself or (b) delegating to a third-party tool (e.g., `cargo`, `make`, or `npm`).

🤔 **Rationale**: X-Launcher provides consistency across projects using different tools and languages by making `x` the universal command to interact with whatever tooling is required to work on a project. In addition, it makes it very easy to add custom freeform tooling in the form of an `xfile` script (e.g., Bash or Python).

Checkout the [examples](https://github.com/silitics/x-launcher/tree/main/examples) to get an idea what X-Launcher can do for you. X-Launcher is designed to work with any tool. Just to name a few examples, X-Launcher works well with [Cargo](https://doc.rust-lang.org/cargo/), [npm](https://www.npmjs.com/), [make](https://www.gnu.org/software/make/), [just](https://just.systems/), and [Poetry](https://python-poetry.org/).


## Installation

Currently, the official way to install X-Launcher is via [Cargo](https://doc.rust-lang.org/cargo/):

```
cargo install x-launcher
```


## Usage

Simply run `x`. All arguments are forwarded to the underlying tool.


## `xfile` Reference

An `xfile` is interpreted in one of two ways:

- If the `xfile` starts with a [Unix shebang](https://en.wikipedia.org/wiki/Shebang_(Unix)), then the `xfile` itself is executed. On Unix-like systems, this is done simply by executing the `xfile` itself. On Windows, `x` takes care of parsing the shebang and executing the correct program with the `xfile` as an argument. To this end, if the shebang starts with a path, other than `/usr/bin/env`, the binary name is looked up using the environment's `PATH` variable. If the shebang starts with `/usr/bin/env` the first argument of `env` is used to lookup the executable.

- If the `xfile` does not start with a shebang, then it must have the form `!command` or `>command` where `command` is interpreted as a [POSIX shell command](https://pubs.opengroup.org/onlinepubs/9699919799/utilities/V3_chap02.html) (currently without performing any expansions, e.g., of variables or `~`). X-Launcher will execute the specified command in the invocation directory, for `!command`, or in the directory of the `xfile`, for `>command`. In the latter case, the environment variable `X_INVOCATION_DIR` is set to the invocation directory such that it can still be retrieved.

In any case, arguments passed to `x` are forwarded to whatever is launched.

Note that in both cases, at least on Unix-like systems, the use of X-Launcher is entirely optional. In case of an `xfile` script with a shebang, the script can still be launched manually, and in case of a command, the command can be directly executed without going through `x`. Hence, X-Launcher is just a convenience.


## Contributing

We do accept bug fixes and improvements, and are open to [discuss ideas](https://github.com/silitics/x-launcher/discussions/categories/ideas). However, note that X-Launcher is intentionally kept very simple. There are already many great tools out there solving different project automation needs. X-Launcher is, as the name suggests, just a launcher and does not aim be more. Hence, we will likely reject any contributions extending X-Launcher beyond the purpose of launching.


## Licensing

X-Launcher is licensed under either [MIT](https://github.com/silitics/sidex/blob/main/LICENSE-MIT) or [Apache 2.0](https://github.com/silitics/sidex/blob/main/LICENSE-APACHE) at your opinion. Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache 2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

Made with ❤️ for OSS by [Silitics](https://www.silitics.com).
