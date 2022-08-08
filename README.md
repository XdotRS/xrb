This repository is home to both XRS and XRB: two Rust library crates developed by the AquariWM project for communication with the [X11 protocol](https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html).

# XRS
### A Rust library for X
XRS is intended to be an API for communication with X11: it focuses on acting as a usable library for X11 projects.

# XRB
### Direct bindings of the X11 spec for Rust
XRB is intended to only be a library for 'X Rust Bindings'; it is a one-to-one direct implementation of the X11 protocol. On its own, XRB does nothing; it simply provides structures mirroring the X11 spec. XRS is a library that builds upon XRB.
