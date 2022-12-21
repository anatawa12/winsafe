use crate::co;

pub(crate) const SECURITY_DESCRIPTOR_REVISION: u32 = 1;
pub(crate) const UNLEN: usize = 256;

#[repr(C)]
#[derive(Clone)]
pub(crate) struct VALENT {
	pub ve_valuename: *mut u16,
	pub ve_valuelen: u32,
	pub ve_valueptr: usize,
	pub ve_type: co::REG,
}

impl_default!(VALENT);

impl VALENT {
	pub(crate) fn buf_projection<'a>(&'a self, src: &'a [u8]) -> &'a [u8] {
		let proj_idx = self.ve_valueptr - src.as_ptr() as usize;
		let proj_past_idx = proj_idx + self.ve_valuelen as usize;
		&src[proj_idx..proj_past_idx]
	}
}
