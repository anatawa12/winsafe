#![allow(non_camel_case_types, non_snake_case)]

use crate::{co, uxtheme};
use crate::gdi::guard::DeleteObjectGuard;
use crate::ole::decl::HrResult;
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::{Handle, IntUnderlying};
use crate::user::decl::{COLORREF, HDC, HRGN, RECT};

impl_handle! { HTHEME;
	/// Handle to a
	/// [theme](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/).
}

impl uxtheme_Htheme for HTHEME {}

/// This trait is enabled with the `uxtheme` feature, and provides methods for
/// [`HTHEME`](crate::HTHEME).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
pub trait uxtheme_Htheme: Handle {
	/// [`DrawThemeBackground`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-drawthemebackground)
	/// function.
	fn DrawThemeBackground(&self,
		hdc: &HDC, part_state: co::VS, rc: RECT, rc_clip: RECT) -> HrResult<()>
	{
		ok_to_hrresult(
			unsafe {
				uxtheme::ffi::DrawThemeBackground(
					self.ptr(),
					hdc.ptr(),
					part_state.part,
					part_state.state,
					&rc as *const _ as _,
					&rc_clip as *const _ as _,
				)
			},
		)
	}

	/// [`GetThemeAppProperties`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemeappproperties)
	/// function.
	#[must_use]
	fn GetThemeAppProperties() -> co::STAP {
		unsafe { co::STAP::from_raw(uxtheme::ffi::GetThemeAppProperties()) }
	}

	/// [`GetThemeBackgroundContentRect`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundcontentrect)
	/// function.
	#[must_use]
	fn GetThemeBackgroundContentRect(&self,
		hdc: &HDC, part_state: co::VS, bounds: RECT) -> HrResult<RECT>
	{
		let mut rc_content = RECT::default();

		ok_to_hrresult(
			unsafe {
				uxtheme::ffi::GetThemeBackgroundContentRect(
					self.ptr(),
					hdc.ptr(),
					part_state.part,
					part_state.state,
					&bounds as *const _ as _,
					&mut rc_content as *mut _ as _,
				)
			},
		).map(|_| rc_content)
	}

	/// [`GetThemeBackgroundExtent`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundextent)
	/// function.
	#[must_use]
	fn GetThemeBackgroundExtent(&self,
		hdc: &HDC, part_state: co::VS, rc_content: RECT) -> HrResult<RECT>
	{
		let mut rc_extent = RECT::default();

		ok_to_hrresult(
			unsafe {
				uxtheme::ffi::GetThemeBackgroundExtent(
					self.ptr(),
					hdc.ptr(),
					part_state.part,
					part_state.state,
					&rc_content as *const _ as _,
					&mut rc_extent as *mut _ as _,
				)
			},
		 ).map(|_| rc_extent)
	}

	/// [`GetThemeBackgroundRegion`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemebackgroundregion)
	/// function.
	#[must_use]
	fn GetThemeBackgroundRegion(&self,
		hdc: &HDC,
		part_state: co::VS,
		rc: RECT,
	) -> HrResult<DeleteObjectGuard<HRGN>>
	{
		let mut hrgn = HRGN::NULL;
		unsafe {
			ok_to_hrresult(
				uxtheme::ffi::GetThemeBackgroundRegion(
					self.ptr(),
					hdc.ptr(),
					part_state.part,
					part_state.state,
					&rc as *const _ as _,
					hrgn.as_mut(),
				),
			).map(|_| DeleteObjectGuard::new(hrgn))
		}
	}

	/// [`GetThemeColor`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-getthemecolor)
	/// function.
	#[must_use]
	fn GetThemeColor(&self,
		part_state: co::VS, prop: co::TMT) -> HrResult<COLORREF>
	{
		let mut color = COLORREF::default();
		ok_to_hrresult(
			unsafe {
				uxtheme::ffi::GetThemeColor(
					self.ptr(),
					part_state.part,
					part_state.state,
					prop.raw(),
					color.as_mut(),
				)
			},
		).map(|_| color)
	}

	/// [`IsThemeBackgroundPartiallyTransparent`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemebackgroundpartiallytransparent)
	/// function.
	#[must_use]
	fn IsThemeBackgroundPartiallyTransparent(&self,
		part_state: co::VS) -> bool
	{
		unsafe {
			uxtheme::ffi::IsThemeBackgroundPartiallyTransparent(
				self.ptr(), part_state.part, part_state.state) != 0
		}
	}

	/// [`IsThemePartDefined`](https://learn.microsoft.com/en-us/windows/win32/api/uxtheme/nf-uxtheme-isthemepartdefined)
	/// function.
	#[must_use]
	fn IsThemePartDefined(&self, part_state: co::VS) -> bool {
		unsafe {
			uxtheme::ffi::IsThemePartDefined(
				self.ptr(), part_state.part, part_state.state) != 0
		}
	}
}
