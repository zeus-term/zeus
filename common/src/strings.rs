use std::ffi::CStr;

/// # Safety
///
/// Use this function at your own risk
pub unsafe fn get_cstr(s: &mut str) -> &CStr {
	let ptr = s.as_mut_ptr() as *const i8;
	CStr::from_ptr(ptr)
}
