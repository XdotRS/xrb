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
(and their serialization/deserialization) for the X Window System protocol v11.
It provides a foundation upon which more opinionated APIs and connection
handling may be built in order to create an 'X library'. In particular, XRB will
serve as the foundation for [X.RS] in the future.

[Rust crate]: https://crates.io/crates/xrb/
[X.RS]: https://github.com/XdotRS/xrs/

> ### Disclaimer
> XRB is not an X library: it cannot be used in an application binary without
> extensive connection logic, nor is it meant to be. Instead, you will be able
> to use [X.RS] in the future - an X library built on XRB.

## Contributing
Contributions are welcome and encouraged for XRB! Please see [CONTRIBUTING] for more information :)

[CONTRIBUTING]: ./CONTRIBUTING.md
