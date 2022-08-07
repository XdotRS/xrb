// This source code form is subject to the terms of the mozilla public
// license, v. 2.0. if a copy of the mpl was not distributed with this
// file, you can obtain one at https://mozilla.org/mpl/2.0/.

/// An ID referring to a particular resource on the X server. Also known as a resource ID.
///
/// The `Xid` can be used in various requests to reference a certain resource, though it is
/// commonly accessed through a [Resource]-implementing struct, such as [`Window`].
pub type Xid = u32;

/// An X resource exists on the X server and can be referenced by its [`Xid`].
///
/// # Example implementation
/// ```rust
/// struct _MyResource {
///     id: xrs::Xid,
/// }
///
/// impl xrs::Resource for _MyResource {
///     fn id(&self) -> xrs::Xid {
///         self.id
///     }
///
///     fn of(id: xrs::Xid) -> Self {
///         Self { id }
///     }
///
///     fn none() -> Self {
///         Self { id: 0 }
///     }
/// }
/// ```
pub trait Resource {
    /// Gets the wrapped [`Xid`].
    ///
    /// # Example usage
    /// ```rust
    /// let xid = window.id();
    /// ```
    fn id(&self) -> Xid;

    /// Wraps an [`Xid`] to instantiate `Self`.
    ///
    /// # Example usage
    /// ```rust
    /// let window = Window::of(conn.generate_xid());
    /// ```
    fn of(id: Xid) -> Self;

    /// Creates a `None` resource ID, which represents a missing resource similarly to
    /// [`Option::None`].
    ///
    /// # Example usage
    /// ```rust
    /// conn.send(xrs::req::FocusWindow {
    ///     window,
    ///     return_mode: xrs::FocusWindowReturnMode::Parent,
    ///     return_to: Window::none(),
    /// });
    /// ```
    fn none() -> Self;

    /// Checks whether this resource is a `None` resource. Shorthand for:
    /// ```rust
    /// self.id() == 0
    /// ```
    ///
    /// # Example usage
    /// ```rust
    /// if !target_window::is_none() {
    ///     // Do the thing.
    /// }
    /// ```
    fn is_none(&self) -> bool {
        self.id() == 0
    }
}

/// Defines a struct type for a type of X resource ID.
///
/// This macro creates a struct with the given identifier that contains an [`Xid`]:
/// ```no_run
/// struct $name {
///     id: Xid,
/// }
/// ```
/// The [Resource] trait is then implemented for that struct:
/// ```no_run
/// impl Resource for $name {
///     // Gets the underlying [`Xid`].
///     fn id(&self) -> Xid {
///         self.id
///     }
///
///     // Creates the struct by wrapping the given [`Xid`].
///     fn of(id: Xid) -> Self {
///         Self { id }
///     }
///
///     // Constructs the struct with the `None` [`Xid`].
///     fn none() -> Self {
///         Self { id: 0 }
///     }
/// }
/// ```
///
/// # Example usage
/// ```rust
/// use xrs::Resource;
///
/// xrs::resource_struct!(Window);
///
/// let window = Window::none();
/// assert!(window.id() == 0);
/// ```
#[macro_export]
macro_rules! resource_struct {
    ($name:ident) => {
        struct $name {
            pub id: Xid,
        }

        impl Resource for $name {
            fn id(&self) -> Xid {
                self.id
            }

            fn of(id: Xid) -> Self {
                Self { id }
            }

            fn none() -> Self {
                Self { id: 0 }
            }
        }
    };
}

resource_struct!(Window);
resource_struct!(Pixmap);
resource_struct!(Cursor);
resource_struct!(Font);
resource_struct!(GraphicsContext);
resource_struct!(Colormap);

/// A marker trait used to represent either a [Window] or a [Pixmap].
trait Drawable {}

impl Drawable for Window {}
impl Drawable for Pixmap {}

/// A fixed-length identifier representing a particular string of text.
///
/// Atoms are used in X to let strings of text be sent in a fixed-length format. An `InternAtom`
/// request can be sent to the X server to generate an `Atom` that represents that string of text.
///
/// # Examples
/// - `WM_PROTOCOLS`
/// - `WM_STATE`
/// - `_NET_WM_STATE`
pub type Atom = u32;
type VisualId = u32;
type Value = u32;
type Timestamp = u32;

enum BitGravity {
    Forget,
    Static,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
}

enum WinGravity {
    Unmap,
    Static,
    NorthWest,
    North,
    NorthEast,
    West,
    Center,
    East,
    SouthWest,
    South,
    SouthEast,
}

type KeySym = u32;
type KeyCode = u8;
type Button = u8;

/// A marker trait used to represent either a [KeyMask] or a [ButtonMask].
trait KeyButtonMask {}

enum KeyMask {
    Shift,
    Lock,
    Control,
    Mod1,
    Mod2,
    Mod3,
    Mod4,
    Mod5,
}

impl KeyButtonMask for KeyMask {}

enum ButtonMask {
    /// The primary mouse button, typically the left mouse button.
    Button1,
    /// The middle mouse button.
    Button2,
    /// The secondary mouse button, typically the right mouse button.
    Button3,
    Button4,
    Button5,
}

impl KeyButtonMask for ButtonMask {}

type Point = (i16, i16);
type Dimensions = (u16, u16);

/// A rectangle that has a position and dimensions, as used in X.
pub struct Rect {
    /// X-coordinate of the rectangle's top-left corner.
    pub x: i16,
    /// Y-coordinate of the rectangle's top-left corner.
    pub y: i16,
    /// Width of the rectangle.
    pub width: u16,
    /// Height of the rectangle.
    pub height: u16,
}

impl Rect {
    /// Gets the position of the rectangle's top-left corner as a [Point].
    pub const fn pos(&self) -> Point {
        (self.x, self.y)
    }

    /// Gets the dimensions of the rectangle as [Dimensions].
    pub const fn dimensions(&self) -> Dimensions {
        (self.width, self.height)
    }
}

enum HostFamily {
    Ipv4,
    Ipv6,
    ServerInterpreted,
    DecNet,
    Chaos,
}

struct Host {
    family: HostFamily,
    address: [u8],
}

pub enum ConnError {
    Access,
    Alloc,
    Atom,
    Colormap,
    Cursor,
    Drawable,
    Font,
    GraphicsContext,
    IdChoice,
    Implementation,
    Match,
    Name,
    Pixmap,
    Request,
    Value,
    Window,
}

pub enum ConnResult<T> {
    Ok(T),
    Err(ConnError),
}
