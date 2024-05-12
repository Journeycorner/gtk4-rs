// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Accessible, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkStackPage")]
    pub struct StackPage(Object<ffi::GtkStackPage>) @implements Accessible;

    match fn {
        type_ => || ffi::gtk_stack_page_get_type(),
    }
}

impl StackPage {
    #[doc(alias = "gtk_stack_page_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> Widget {
        unsafe { from_glib_none(ffi::gtk_stack_page_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_page_get_icon_name")]
    #[doc(alias = "get_icon_name")]
    pub fn icon_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_stack_page_get_icon_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_page_get_name")]
    #[doc(alias = "get_name")]
    pub fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_stack_page_get_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_page_get_needs_attention")]
    #[doc(alias = "get_needs_attention")]
    pub fn needs_attention(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_page_get_needs_attention(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_stack_page_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_stack_page_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_page_get_use_underline")]
    #[doc(alias = "get_use_underline")]
    pub fn uses_underline(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_page_get_use_underline(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_page_get_visible")]
    #[doc(alias = "get_visible")]
    pub fn is_visible(&self) -> bool {
        unsafe { from_glib(ffi::gtk_stack_page_get_visible(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_stack_page_set_icon_name")]
    pub fn set_icon_name(&self, setting: &str) {
        unsafe {
            ffi::gtk_stack_page_set_icon_name(self.to_glib_none().0, setting.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_stack_page_set_name")]
    pub fn set_name(&self, setting: &str) {
        unsafe {
            ffi::gtk_stack_page_set_name(self.to_glib_none().0, setting.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_stack_page_set_needs_attention")]
    pub fn set_needs_attention(&self, setting: bool) {
        unsafe {
            ffi::gtk_stack_page_set_needs_attention(self.to_glib_none().0, setting.into_glib());
        }
    }

    #[doc(alias = "gtk_stack_page_set_title")]
    pub fn set_title(&self, setting: &str) {
        unsafe {
            ffi::gtk_stack_page_set_title(self.to_glib_none().0, setting.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_stack_page_set_use_underline")]
    pub fn set_use_underline(&self, setting: bool) {
        unsafe {
            ffi::gtk_stack_page_set_use_underline(self.to_glib_none().0, setting.into_glib());
        }
    }

    #[doc(alias = "gtk_stack_page_set_visible")]
    pub fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::gtk_stack_page_set_visible(self.to_glib_none().0, visible.into_glib());
        }
    }

    #[doc(alias = "icon-name")]
    pub fn connect_icon_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_name_trampoline<F: Fn(&StackPage) + 'static>(
            this: *mut ffi::GtkStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_icon_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "name")]
    pub fn connect_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<F: Fn(&StackPage) + 'static>(
            this: *mut ffi::GtkStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "needs-attention")]
    pub fn connect_needs_attention_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_needs_attention_trampoline<F: Fn(&StackPage) + 'static>(
            this: *mut ffi::GtkStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::needs-attention\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_needs_attention_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&StackPage) + 'static>(
            this: *mut ffi::GtkStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-underline")]
    pub fn connect_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_underline_trampoline<F: Fn(&StackPage) + 'static>(
            this: *mut ffi::GtkStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-underline\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_use_underline_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible")]
    pub fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<F: Fn(&StackPage) + 'static>(
            this: *mut ffi::GtkStackPage,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_visible_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
