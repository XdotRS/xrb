**X Rust Bindings** is an implementation of the X Window System protocol version 11, more commonly known as X11, in Rust. It provides types and data structures for X11, as well as serialization and deserialization for them.

XRB is not, however, an X library. It does not offer functionality for connecting to the X server, sending or receiving messages, nor an opinionated API. The idea is that those functionalities are implemented on top of XRB with an _API wrapper_, like [X.RS](https://github.com/XdotRS/xrs).
