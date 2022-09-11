**X Rust Bindings** is an implementation of the X Window System protocol version
11, more commonly known as X11, in Rust. It provides types and data structures
for X11, as well as serialization and deserialization for them.

XRB is not, however, an X library. It does not offer functionality for connecting
to the X server, sending or receiving messages, nor an opinionated API. The idea
is that those functionalities are implemented on top of XRB with an API wrapper,
such as [X.RS](https://github.com/XdotRS/xrs). XRB provides a foundation for API
wrappers so that they can focus on their APIs, not on implementing the X protocol.

# Contributing
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
