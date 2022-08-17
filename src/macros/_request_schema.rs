use crate::proto::{atoms::Atom, ids::Window};

macro_rules! schema {
	(
		$($_name:ident)? {
			$vis:vis struct $request:ident$(<$req_life:lifetime>)? {
				header {
					1; u8; $code:expr; opcode;

					$meta_len:expr;
					$meta_ty:ty$(: {
						$($meta_alt:ident = $meta_alt_val:expr;)+
					})?;
					$($meta_val:expr)?;
					$meta:ident;

					2; u16; $len:expr; length;
				}

				data {
					$(
						$data_len:expr;
						$data_ty:ty$(: {
							$($alt:ident = $alt_val:expr;)+
						})?;
						$($data_val:expr)?;
						$data:ident;
					)+
				}
			}

			$(
				$rep_vis:vis struct $reply:ident$(<$rep_life:lifetime>)? {
					header {
						1; u8; 1; reply;

						$rep_meta_len:expr;
						$rep_meta_ty:ty$(: {
							$($rep_meta_alt:ident = $rep_meta_alt_val:expr;)+
						})?;
						$($rep_meta_val:expr)?;
						$rep_meta:ident;

						2; u16; ; sequence_number;
						4; u32; $rep_len:expr; length;
					}

					data {
						$(
							$rep_data_len:expr;
							$rep_data_ty:ty$(: {
								$($rep_alt:ident = $rep_alt_val:expr;)+
							})?;
							$($rep_data_val:expr)?;
							$rep_data:ident;
						)*
					}
				}
			)?
		}
	) => {
		pub struct $request$(<$req_life>)? {
			$meta: $meta_ty,
			length: u16,
			$(pub $data: $data_ty,)+
		}

		$($(pub const $meta_alt: $meta_ty = $meta_alt_val;)+)?
		$($($(pub const $alt: $data_ty = $alt_val;)+)?)+

		impl$(<$req_life>)? $request$(<$req_life>)? {
			fn opcode() -> u8 {
				$code
			}

			fn metadata(&self) -> $meta_ty {
				self.$meta
			}

			fn length(&self) -> u16 {
				$len
			}
		}

		$(
			pub struct $reply$(<$rep_life>)? {
				$rep_meta: $rep_meta_ty,
				sequence_number: u16,
				length: u32,
				$(pub $rep_data: $rep_data_ty,)*
			}

			$($(pub const $rep_meta_alt: $rep_meta_ty = $rep_meta_alt_val;)+)?
			$($($(pub const $rep_alt: $rep_data_ty = $rep_alt_val;)+)?)*

			impl$(<$rep_life>)? $reply$(<$rep_life>)? {
				fn metadata(&self) -> $rep_meta_ty {
					self.$rep_meta
				}

				fn sequence_number(&self) -> u16 {
					self.sequence_number
				}

				fn length(&self) -> u32 {
					self.length
				}
			}
		)?
	};
}

schema! {
	GetProperty {
		pub struct GetPropertyRequest {
			header {
			//	bytes;	type;		value;	name;
				1;		u8;			20;		opcode;
				1;		bool;		;		delete;
				2;		u16;		6;		length;
			}

			data {
			//	bytes;	type;		value;	name;
				4;		Window;		;		window;
				4;		Atom;		;		property;
				4;		Atom: {
					ANY_PROPERTY_TYPE = 0;
				};					;		ty;
				4;		u32;		;		long_offset;
				4;		u32;		;		long_length;
			}
		}

		pub struct GetPropertyReply<'a> {
			header {
			//	bytes;	type;		value;	name;
				1;		u8;			1;		reply;
				1;		u8;			4;		format;
				2;		u16;		;		sequence_number;

				4;		u32;		8;		length;
			}

			data {
			//	bytes;	type;		value;	name;
				4;		Atom: {
					NONE = 0;
				};					;		ty;
				4;		u32;		;		bytes_after;
				4;		u32;		4;		val_length;
				4;		&'a [u8];	;		value;
				4;	u8;				;		x;
			}
		}
	}
}
