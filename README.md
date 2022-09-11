<!-- This Source Code Form is subject to the terms of the Mozilla Public
   - License, v. 2.0. If a copy of the MPL was not distributed with this
   - file, You can obtain one at https://mozilla.org/MPL/2.0/. -->

# X Rust Bindings
X Rust Bindings (a.k.a. XRB) is an implementation of the X Window System protocol version
11, more commonly known as X11, in Rust. It provides types and data structures
for X11, as well as serialization and deserialization for them.

> _Hey! Why not check out [XRB's documentation](https://docs.aquariwm.org/doc/xrb)?
> It contains a lot of information about the project, and has many examples and
> explanations._

XRB is not, however, an X library. It does not offer functionality for connecting
to the X server, sending or receiving messages, nor an opinionated API. The idea
is that those functionalities are implemented on top of XRB with an API wrapper,
such as [X.RS](https://github.com/XdotRS/xrs). XRB provides a foundation for API
wrappers so that they can focus on their APIs, not on implementing the X protocol.

## Contributing
Contributions are welcome and encouraged for XRB! Here's a list of resources that
you may find useful:
 - [X Window System protocol version 11](https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html)
   – The protocol itself.
   - [1. Protocol Formats](https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html#Protocol_Formats)
     – An overview of the format of messages in the X11 protocol.
   - [Appendix B. Protocol Encoding](https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html#protocol_encoding)
     – The encoding of X11 types and data structures as bytes. Probably the most
	 directly important section for the development of XRB.
   - [Glossary](https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html#glossary)
     – A glossary of terms used in X, helpful to understand what's going on and
	 especially to write documentation.
 - [The Rust Programming Language](https://doc.rust-lang.org/book/) – A great
   resource to learn Rust in general.
 - [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) –
   Recommendations for the design and presentation of Rust APIs.
 - [Mozilla License Headers](https://www.mozilla.org/en-US/MPL/headers/) –
   Copy-and-pasteable headers the MPL-v2.0 license. This header must be added to
   every source file in XRB, preferably as a comment at the beginning of the file.
