// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// A raw bitmask value that indicates the presence of certain fields.
type Mask = u32;
/// A _resource ID_ that can be used to specify a particular window.
type Window = u32;
/// A _resoruce ID_ that can be used to specify a particular pixmap (a.k.a. texture).
type Pixmap = u32;
/// A _resource ID_ that can be used to specify a particular cursor appearance.
///
/// For example, the 'arrow' appearance of the cursor may be represented by a
/// [Cursor] resource ID.
type Cursor = u32;
/// A _resource ID_ that can be used to specify a particular system font.
type Font = u32;
/// A _resource ID_ that can be used to specify a particular gcontext.
///
/// TODO: What's a gcontext?
type Gcontext = u32;
/// A _resource ID_ that can be used to specify a particular colormap.
///
/// A colormap can be thought of as a palette of colors - it allows a limited
/// number of colors to be represented with a lower color depth than they might
/// ordinarily use.
type Colormap = u32;
/// A _resource ID_ that can be used to specify either a [Window] or a [Pixmap].
type Drawable = u32;
/// A _resource ID_ that can be used to specify either a [Font] or a [Gcontext].
type Fontable = u32;
/// An ID representing a string of text that has been registered with the X server.
///
/// An [Atom] provides a fixed-length representation of what may be a longer
/// string of text. It allows messages, such as requests, to remain a fixed
/// length, even if the text that has been registered with the X server is longer
/// than four bytes.
type Atom = u32;
/// An ID representing a 'visual'.
///
/// TODO: What is a visual?
type VisualId = u32;
/// A timestamp expressed in milliseconds, typically since the last server reset.
type Timestamp = u32;
