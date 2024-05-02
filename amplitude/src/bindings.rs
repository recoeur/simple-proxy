// Generated by `wit-bindgen` 0.21.0. DO NOT EDIT!
// Options used:
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_page_viewed_cabi<T: Guest>(arg0: *mut u8, arg1: usize) {
    let len0 = arg1;
    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
    T::page_viewed(_rt::string_lift(bytes0));
}
pub trait Guest {
    fn page_viewed(url: _rt::String);
}
#[doc(hidden)]

macro_rules! __export_world_event_handler_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "page-viewed"]
    unsafe extern "C" fn export_page_viewed(arg0: *mut u8,arg1: usize,) {
      $($path_to_types)*::_export_page_viewed_cabi::<$ty>(arg0, arg1)
    }
  };);
}
#[doc(hidden)]
pub(crate) use __export_world_event_handler_cabi;
mod _rt {
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_event_handler_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::__export_world_event_handler_cabi!($ty with_types_in $($path_to_types_root)*);
  )
}
#[doc(inline)]
pub(crate) use __export_event_handler_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.21.0:event-handler:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 197] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07B\x01A\x02\x01A\x02\x01\
@\x01\x03urls\x01\0\x04\0\x0bpage-viewed\x01\0\x04\x01\x1dedgee:amplitude/event-\
handler\x04\0\x0b\x13\x01\0\x0devent-handler\x03\0\0\0G\x09producers\x01\x0cproc\
essed-by\x02\x0dwit-component\x070.201.0\x10wit-bindgen-rust\x060.21.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
