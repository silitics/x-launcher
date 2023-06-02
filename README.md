# X-Launcher 🚀

A cross-platform tool-agnostic project automation launcher.

**TL;DR**: The X-Launcher is a simple binary `x` which, when executed, scans for an `xfile` starting in the invocation directory and working its way up. When it finds an `xfile`, it uses it to launch project-specific tooling either (a) by executing the `xfile` itself or (b) delegating to a third-party tool (e.g., `cargo`, `make`, or `npm`).

🤔 **Rationale**: The X-Launcher provides consistency across projects using different tools and languages by making `x` the command to interact with whatever tooling is required to work on the project. In addition, it makes it easy to add custom freeform tooling in the form of an `xfile` script (e.g., Bash or Python).

Checkout the [examples](https://github.com/silitics/x-launcher/tree/main/examples) to get an idea what X-Launcher can do for you. Among other tools, the X-Launcher works well with [Cargo](https://doc.rust-lang.org/cargo/), [npm](https://www.npmjs.com/), [make](https://www.gnu.org/software/make/), [just](https://just.systems/), and [Poetry](https://python-poetry.org/).


## `xfile` Reference

The first line of an `xfile` determines how it is interpreted.

**Shebang**: If the first line is a Unix shebang of the form `#!/path/to/an/executable args...`, then the `xfile` itself is executed. On Windows, the shebang is parsed and the name of the executable is resolved via the environments `PATH` variable (except if the path is `/usr/bin/env` in which case the first argument is used).


## Installation

Via Cargo:

```
cargo install x-launcher
```
