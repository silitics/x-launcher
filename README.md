# X-Launcher 🚀

A cross-platform tool-agnostic project automation launcher.

**TL;DR**: A simple binary `x` which, when executed, scans for an `xfile` starting in the invocation directory and working its way up. When it finds an `xfile`, it uses it to launch project-specific tooling either (a) by executing the `xfile` itself or (b) invoking a third-party tool (e.g., `cargo`, `make`, or `npm`).
