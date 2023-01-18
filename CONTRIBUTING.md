<!-- This Source Code Form is subject to the terms of the Mozilla Public
   - License, v. 2.0. If a copy of the MPL was not distributed with this
   - file, You can obtain one at https://mozilla.org/MPL/2.0/. -->

[![GitHub pull requests](https://img.shields.io/github/issues-pr-raw/XdotRS/xrb?color=%23a060d8&label=Open%20PRs&style=for-the-badge)](https://github.com/XdotRS/xrb/pulls)
[![Issues](https://img.shields.io/github/issues-raw/XdotRS/xrb?style=for-the-badge)](https://github.com/XdotRS/xrb/issues)
[![CI status](https://img.shields.io/github/actions/workflow/status/XdotRS/xrb/ci.yml?event=push&branch=main&label=ci&style=for-the-badge)](https://github.com/XdotRS/xrb/actions/workflows/ci.yml)

[![License: MPL-2.0](https://img.shields.io/crates/l/xrb?style=for-the-badge)](https://github.com/XdotRS/xrb/blob/main/LICENSE)
[![Documentation (dev build)](https://img.shields.io/badge/docs-dev%20build-forestgreen?style=for-the-badge)](https://docs.aquariwm.org/doc/xrb/)
[![XRB GitHub project](https://img.shields.io/badge/todo-project-303048?style=for-the-badge)](https://github.com/orgs/XdotRS/projects/1/views/1)

# Contributing to XRB
First of all, thank you for looking into contributing to XRB.
XRB is an ambitious project, designed to be community-driven from the beginning.
Both users and contributors are the heart of any community-driven project, and while
XRB might not yet be ready for users, contributions can help bring that goal much
closer!

## Ask questions!
If you ever have any questions about contributing, whether that be about the X11
protocol, about Rust, about how to submit contributions, or anything else, please
feel free to ask! You can ask questions by either creating a [discussion] here on
GitHub, or by asking in the #xrb-dev-chat channel in [the Discord server]!

[discussion]: https://github.com/XdotRS/xrb/discussions
[the Discord server]: https://discord.gg/CmsZBEsf5N

## Interpreting [the X11 protocol]
Since XRB is an implementation of the messages, types, and data structures found
in [the X11 protocol], you're probably going to need to understand the protocol for
many areas of working on XRB. That's where this section comes in.

[the X11 protocol]: https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html

### [Glossary][glossary]
Firstly, the X11 protocol has a [glossary] section at the end which explains many of
the terms used both throughout the protocol and throughout XRB.

Note that there are some terms which we have opted to rename in XRB to make them
clearer, especially for modern conventions and jargon. One such example is that
in the X11 protocol, 'pointer' refers to what we would now call the cursor, while
'cursor' refers to the appearance of the cursor on the screen.

[glossary]: https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html#glossary

### [Protocol encoding][protocol encoding]
The [protocol encoding] section of the X11 protocol is one of the most important
sections relevant to the development of XRB. It defines the encoding of various
types used throughout the X11 protocol in bytes. It does, however, make some
rather... odd... choices, at times, with how these types are encoded, as well as
how that encoding is presented.

#### Components
'Component' refers to something encoded. That might be a field, or it might be
some other data, such as the length of a message, the code uniquely identifying
its type of message, etc.

In the [protocol encoding] section, each component's encoding is written like
so:
```
N	TYPE	component_name
```
...unless that component has a numeric value which can be determined from the
rest of the components, or is a constant, in which case it is written as:
```
N	value	component_name
```

In these examples, `N` refers to the number of bytes the component takes up,
`TYPE` refers to the type of the data, and `value` refers to an expression to
determine the data. `component_name` refers to the name of the component.

Typically, in the [macro syntax] used in XRB, the former represent fields, and
the latter represent 'let elements'. For example, this:
```
4	WINDOW	parent
```
is converted to a field in the macro syntax:
```rust
pub parent: Window,
```
While this:
```
4       m               number of CHARINFOs in char-infos
12m     LISTofCHARINFO  char-infos
```
is converted into:
```rust
let charinfos_len: u32 = charinfos => charinfos.len() as u32,

#[context(charinfos_len => *charinfos_len as usize)]
pub charinfos: Vec<CharInfo>,
```

The [protocol encoding] section is a complex topic to cover, as is the use of
the macro syntax, so this section of the contributing guide is yet to be
fleshed out. For now, you can look at types in the [protocol encoding]
section and try to find their equivalents in XRB's codebase. Some of the best
examples would be comparing the events in the events section of the
[protocol encoding] section to the events defined in [`x11::event`].

[macro syntax]: https://docs.aquariwm.org/doc/xrbk_macro/macro.derive_xrb.html
[protocol encoding]: https://x.org/releases/X11R7.7/doc/xproto/x11protocol.html#protocol_encoding
[`x11::event`]: https://github.com/XdotRS/xrb/blob/main/src/x11/event.rs

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
