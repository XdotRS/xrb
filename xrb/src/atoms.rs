// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! [Atom](crate::Atom) constants that are predefined in the X11 protocol.
//!
//! The X11 protocol predefines the values of the following 68 [Atom](crate::Atom)s for
//! convenience. These [Atom](crate::Atom)s always have the same value, across all libraries, all
//! connections, and all clients. Other [Atom](crate::Atom)s, i.e. [Atom](crate::Atom)s not listed
//! here, and which are not predefined, are dependent on the connection: the only way to know what
//! these other [Atom](crate::Atom)s' values are for any given connection is to send
//! [InternAtom](crate::requests::InternAtom) requests for them.
//!
//! Once one [InternAtom](crate::replies::InternAtom) reply has been received, however, you can
//! assume it will remain the same for the duration of the current connection.

use crate::predefined_atoms;

predefined_atoms!(
    PRIMARY,
    SECONDARY,
    /// Associated type: [Arc](crate::Arc)
    ARC,
    /// Associated type: [Atom](crate::Atom)
    ATOM,
    BITMAP,
    CARDINAL,
    /// Associated type: [Colormap](crate::Colormap)
    COLORMAP,
    /// Associated type: [Cursor](crate::Cursor)
    CURSOR,
    CUT_BUFFER0,
    CUT_BUFFER1,
    CUT_BUFFER2,
    CUT_BUFFER3,
    CUT_BUFFER4,
    CUT_BUFFER5,
    CUT_BUFFER6,
    CUT_BUFFER7,
    /// Associated type: [Drawable](crate::Drawable)
    DRAWABLE,
    /// Associated type: [Font](crate::Font)
    FONT,
    INTEGER,
    /// Associated type: [Pixmap](crate::Pixmap)
    PIXMAP,
    /// Associated type: [Point](crate::Point)
    POINT,
    /// Associated type: [Rect](crate::Rect)
    RECTANGLE,
    RESOURCE_MANAGER,
    RGB_COLOR_MAP,
    RGB_BEST_MAP,
    RGB_BLUE_MAP,
    RGB_DEFAULT_MAP,
    RGB_GRAY_MAP,
    RGB_GREEN_MAP,
    RGB_RED_MAP,
    /// Associated type: [String]
    STRING,
    /// Associated type: [VisualId](crate::VisualId)
    VISUALID,
    /// Associated type: [Window](crate::Window)
    WINDOW,
    WM_COMMAND,
    WM_HINTS,
    WM_CLIENT_MACHINE,
    WM_ICON_NAME,
    WM_ICON_SIZE,
    WM_NAME,
    WM_NORMAL_HINTS,
    WM_SIZE_HINTS,
    WM_ZOOM_HINTS,
    MIN_SPACE,
    NORM_SPACE,
    MAX_SPACE,
    END_SPACE,
    SUPERSCRIPT_X,
    SUPERSCRIPT_Y,
    SUBSCRIPT_X,
    SUBSCRIPT_Y,
    UNDERLINE_POSITION,
    UNDERLINE_THICKNESS,
    STRIKEOUT_ASCENT,
    STRIKEOUT_DESCENT,
    ITALIC_ANGLE,
    X_HEIGHT,
    QUAD_WIDTH,
    WEIGHT,
    POINT_SIZE,
    RESOLUTION,
    COPYRIGHT,
    NOTICE,
    FONT_NAME,
    FAMILY_NAME,
    FULL_NAME,
    CAP_HEIGHT,
    WM_CLASS,
    WM_TRANSIENT_FOR
);
