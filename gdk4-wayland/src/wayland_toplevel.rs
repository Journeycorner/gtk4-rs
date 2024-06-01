// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ffi, WaylandToplevel};
use glib::translate::*;
use std::boxed::Box as Box_;

impl WaylandToplevel {
    #[doc(alias = "gdk_wayland_toplevel_export_handle")]
    pub fn export_handle<P: Fn(&WaylandToplevel, Result<&str, glib::BoolError>) + 'static>(
        &self,
        callback: P,
    ) -> bool {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<
            P: Fn(&WaylandToplevel, Result<&str, glib::BoolError>) + 'static,
        >(
            toplevel: *mut ffi::GdkWaylandToplevel,
            handle: *const libc::c_char,
            user_data: glib::ffi::gpointer,
        ) {
            let toplevel = from_glib_borrow(toplevel);
            let handle: Borrowed<Option<glib::GString>> = from_glib_borrow(handle);
            let callback = &*(user_data as *mut P);
            if let Some(handle) = handle.as_ref() {
                (*callback)(&toplevel, Ok(handle.as_str()))
            } else {
                (*callback)(&toplevel, Err(glib::bool_error!("Failed to export a handle. The compositor probably doesn't implement the xdg-foreign protocol")))
            }
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn destroy_func_func<
            P: Fn(&WaylandToplevel, Result<&str, glib::BoolError>) + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback = Box_::from_raw(data as *mut P);
        }
        let destroy_call3 = Some(destroy_func_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            from_glib(ffi::gdk_wayland_toplevel_export_handle(
                self.to_glib_none().0,
                callback,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            ))
        }
    }
}
