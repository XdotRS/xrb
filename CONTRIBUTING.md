<!-- This Source Code Form is subject to the terms of the Mozilla Public
   - License, v. 2.0. If a copy of the MPL was not distributed with this
   - file, You can obtain one at https://mozilla.org/MPL/2.0/. -->

[![Documentation (dev build)](https://img.shields.io/badge/docs-dev%20build-forestgreen?style=for-the-badge)](https://docs.aquariwm.org/doc/xrb/)
[![XRB GitHub project](https://img.shields.io/badge/todo-project-8860b8?style=for-the-badge)](https://github.com/orgs/XdotRS/projects/1/views/1)

[![License: MPL-2.0](https://img.shields.io/crates/l/xrb?style=for-the-badge)](https://github.com/XdotRS/xrb/blob/main/LICENSE)
[![Issues](https://img.shields.io/github/issues-raw/XdotRS/xrb?style=for-the-badge)](https://github.com/XdotRS/xrb/issues)
[![CI status](https://img.shields.io/github/actions/workflow/status/XdotRS/xrb/ci.yml?event=push&branch=main&label=ci&style=for-the-badge)](https://github.com/XdotRS/xrb/actions/workflows/ci.yml)

# Contributing to XRB
First of all, thank you for looking into contributing to XRB.
XRB is an ambitious project, designed to be community-driven from the beginning.
Both users and contributors are the heart of any community-driven project, and while
XRB might not yet be ready for users, contributions can help bring that goal much
closer!

## Ask questions!
TODO <!-- (add Discord server URL) -->

## Interpreting the X11 protocol
TODO

### Glossary
TODO

### Protocol encoding
TODO

## Code style
TODO

## Useful resources
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
