X11 protocol formats, structures, serialization, and deserialization.

# X Rust Bindings
X Rust Bindings is a Rust library for the X Window System protocol version 11.
It is _not_ a full API: XRB simply provides a mirror of the formats of X11
protocol messages in the form of Rust `struct` and `enum` datastructures. It
also provides tools for the serialization and deserialization of these messages
to and from their format when sent along 'the wire', i.e., their underlying
representations as raw bytes.

On its own, XRB does nothing. It is only useful when paired with a full X
library API wrapping. The official API wrapper for XRB is
[X.RS](https://github.com/XdotRS/xrs), but it may be freely used in the
creation of any other X library, provided that the terms of the [Mozilla Public
License v2.0](https://github.com/XdotRS/xrb/blob/main/LICENSE) are respected.
