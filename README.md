<!-- This Source Code Form is subject to the terms of the Mozilla Public
   - License, v. 2.0. If a copy of the MPL was not distributed with this
   - file, You can obtain one at https://mozilla.org/MPL/2.0/. -->

<h1 align="center">X Rust Bindings</h1>
<p align="center">
	<a href="https://github.com/XdotRS/xrb/blob/main/LICENSE">
		<img src="https://img.shields.io/crates/l/xrb?style=for-the-badge" /></a>
	<a href="https://crates.io/crates/xrb">
		<img src="https://img.shields.io/crates/v/xrb?style=for-the-badge" /></a>
	<a href="https://github.com/XdotRS/xrb/issues">
		<img src="https://img.shields.io/github/issues-raw/XdotRS/xrb?style=for-the-badge" /></a>
	<a href="https://github.com/XdotRS/xrb/actions/workflows/ci.yml">
		<img src="https://img.shields.io/github/actions/workflow/status/XdotRS/xrb/ci.yml?event=push&branch=main&label=ci&style=for-the-badge" /></a>
</p>
<p align="center">
	<a href="https://docs.aquariwm.org/doc/xrb/">
		<img src="https://img.shields.io/badge/docs-dev build-forestgreen?style=for-the-badge" /></a>
	<a href="https://github.com/orgs/XdotRS/projects/1/views/1">
		<img src="https://img.shields.io/badge/todo-project-8860b8?style=for-the-badge" /></a>
</p>

X Rust Bindings, better known as XRB, is a [Rust crate] implementing data structures
(and their serialization/deserialization) for the X Window System protocol version
11. It provides a foundation upon which more opinionated APIs and connection
handling may be built in order to create an 'X library'.

XRB serves as a foundation for [X.RS (WIP)][X.RS] in particular.

[Rust crate]: https://crates.io/crates/xrb/
[X.RS]: https://github.com/XdotRS/xrs/

> ### Disclaimer
> XRB is not an X library: it cannot be used in an application binary without
> extensive connection logic, nor is it meant to be. Instead, you can use an X
> library built on XRB such as [X.RS (WIP)][X.RS].

## Contributing
Contributions are welcome and encouraged for XRB! You can see tasks that need to be done (see, in particular, the 'Unassigned' column) on the [X Rust Bindings project][XRB project]. Additionally, here's a list of resources that you may find useful:
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

[XRB project]: https://github.com/orgs/XdotRS/projects/1/views/1
