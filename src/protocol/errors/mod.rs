// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[macro_use]
mod error_macro;

use std::error::Error;

/// An X protocol error that can be returned when sending requests.
pub trait Xerror: Error {
	/// The unique error code for this type of error.
	fn code(&self) -> u8;
	/// The sequence of the last associated request sent to the X server.
	fn sequence(&self) -> u16;
	/// The minor opcode of the last associated request.
	fn minor_opcode(&self) -> u16;
	/// The major opcode of the last associated request.
	fn major_opcode(&self) -> u8;
}

// NOTE: While these might be error messages of some kind, this will not be an
// acceptable standard of error handling for X.RS. Work will need to be done to
// keep track of the context when in debug environments so that more accurate
// and detailed messages can be given. These messages will have to be linked to
// their function calls directly, and they should be able to tell you exactly
// what's wrong, where, and how to fix it. I don't know whether X.RS will work
// around these simple implementations in XRB, or whether XRB will be more
// involved in that.

// Automatically generate error structs. This is not an enum: external errors can
// always be added at any time. It's just a convenience macro for defining many
// errors.
errors! {
	#[error("the major or minor opcode does not specify a valid request")]
	pub struct RequestXerror(1) {}
	#[error("`{bad_value:?}` falls outside the range of values accepted by this request")]
	pub struct ValueXerror(2) { bad_value }
	#[error("`{bad_res_id:?}` is not a defined window resource ID")]
	pub struct WindowXerror(3) { bad_res_id }
	#[error("`{bad_res_id:?}` is not a defined pixmap resource ID")]
	pub struct PixmapXerror(4) { bad_res_id }
	#[error("`{bad_atom_id:?}` is not a defined atom ID")]
	pub struct AtomXerror(5) { bad_atom_id }
	#[error("`${bad_res_id:?}` is not a defined cursor resource ID")]
	pub struct CursorXerror(6) { bad_res_id }
	#[error("`${bad_res_id:?}` is not a defined font resource ID, or font or gcontext resource ID")]
	pub struct FontXerror(7) { bad_res_id }
	#[error("a given argument or arguments did not match a valid value")]
	pub struct MatchXerror(8) {}
	#[error("`${bad_res_id:?}` is not a defined window or pixmap resource ID")]
	pub struct DrawableXerror(9) { bad_res_id }
	#[error("unauthorized access")]
	pub struct AccessXerror(10) {}
	#[error("failed to allocate the requested resource")]
	pub struct AllocXerror(11) {}
	#[error("`{bad_res_id:?}` is not a defined colormap resource ID")]
	pub struct ColormapXerror(12) { bad_res_id }
	#[error("`{bad_res_id:?}` is not a defined gcontext resource ID")]
	pub struct GcontextXerror(13) { bad_res_id }
	#[error("the chosen resource ID is either already in use or it is not assigned to this client")]
	pub struct IdChoiceXerror(14) { bad_res_id }
	#[error("a font or color of the specified name does not exist")]
	pub struct NameXerror(15) {}
	#[error("the length of a request is too short or too long")]
	pub struct LengthXerror(16) {}
	#[error("the X server does not support some aspect of this request")]
	pub struct ImplementationXerror(17) {}
}
