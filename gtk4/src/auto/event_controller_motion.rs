// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{EventController, PropagationLimit, PropagationPhase};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkEventControllerMotion")]
    pub struct EventControllerMotion(Object<ffi::GtkEventControllerMotion, ffi::GtkEventControllerMotionClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_event_controller_motion_get_type(),
    }
}

impl EventControllerMotion {
    #[doc(alias = "gtk_event_controller_motion_new")]
    pub fn new() -> EventControllerMotion {
        assert_initialized_main_thread!();
        unsafe {
            EventController::from_glib_full(ffi::gtk_event_controller_motion_new()).unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`EventControllerMotion`] objects.
    ///
    /// This method returns an instance of [`EventControllerMotionBuilder`](crate::builders::EventControllerMotionBuilder) which can be used to create [`EventControllerMotion`] objects.
    pub fn builder() -> EventControllerMotionBuilder {
        EventControllerMotionBuilder::new()
    }

    #[doc(alias = "gtk_event_controller_motion_contains_pointer")]
    pub fn contains_pointer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_motion_contains_pointer(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_event_controller_motion_is_pointer")]
    pub fn is_pointer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_event_controller_motion_is_pointer(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "enter")]
    pub fn connect_enter<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn enter_trampoline<F: Fn(&EventControllerMotion, f64, f64) + 'static>(
            this: *mut ffi::GtkEventControllerMotion,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"enter\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    enter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "leave")]
    pub fn connect_leave<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn leave_trampoline<F: Fn(&EventControllerMotion) + 'static>(
            this: *mut ffi::GtkEventControllerMotion,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"leave\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    leave_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "motion")]
    pub fn connect_motion<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn motion_trampoline<
            F: Fn(&EventControllerMotion, f64, f64) + 'static,
        >(
            this: *mut ffi::GtkEventControllerMotion,
            x: libc::c_double,
            y: libc::c_double,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this), x, y)
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"motion\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    motion_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "contains-pointer")]
    pub fn connect_contains_pointer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_contains_pointer_trampoline<
            F: Fn(&EventControllerMotion) + 'static,
        >(
            this: *mut ffi::GtkEventControllerMotion,
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
                b"notify::contains-pointer\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_contains_pointer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "is-pointer")]
    pub fn connect_is_pointer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_is_pointer_trampoline<
            F: Fn(&EventControllerMotion) + 'static,
        >(
            this: *mut ffi::GtkEventControllerMotion,
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
                b"notify::is-pointer\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_is_pointer_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl Default for EventControllerMotion {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`EventControllerMotion`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct EventControllerMotionBuilder {
    builder: glib::object::ObjectBuilder<'static, EventControllerMotion>,
}

impl EventControllerMotionBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn propagation_limit(self, propagation_limit: PropagationLimit) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-limit", propagation_limit),
        }
    }

    pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-phase", propagation_phase),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`EventControllerMotion`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> EventControllerMotion {
        self.builder.build()
    }
}
