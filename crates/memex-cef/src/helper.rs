use std::marker::PhantomData;

use cef::{ThreadId, sys::cef_thread_id_t};

#[inline]
pub fn tid_ui() -> ThreadId {
    ThreadId::from(cef_thread_id_t::TID_UI)
}

#[derive(Copy, Clone)]
pub struct UIThreadMarker {
    _priv: PhantomData<*mut ()>,
}

impl UIThreadMarker {
    pub fn new() -> Option<Self> {
        if cef::currently_on(tid_ui()) == 1 {
            Some(Self { _priv: PhantomData })
        } else {
            None
        }
    }

    /// # Safety
    /// You must ensure that the current thread is UI thread.
    pub unsafe fn new_unchecked() -> Self {
        Self { _priv: PhantomData }
    }
}

pub mod __macros {
    #[macro_export]
    macro_rules! impl_wrap_object {
        ($wrap_object:ty, $struct:ident, $object_raw:ty) => {
            impl $wrap_object for $struct {
                fn wrap_rc(&mut self, object: *mut cef::rc::RcImpl<$object_raw, Self>) {
                    self.sys = object;
                }
            }
        };
    }

    #[macro_export]
    macro_rules! impl_rc {
        ($struct:ident) => {
            impl cef::rc::Rc for $struct {
                fn as_base(&self) -> &cef::sys::cef_base_ref_counted_t {
                    unsafe {
                        let base = &*self.sys;
                        std::mem::transmute(&base.cef_object)
                    }
                }
            }
        };
    }

    #[macro_export]
    macro_rules! impl_clone {
        ($struct:ident, [$($field:ident),*]) => {
            impl Clone for $struct {
                fn clone(&self) -> Self {
                    let sys = unsafe {
                        use cef::rc::Rc;

                        let rc_impl = &mut *self.sys;
                        rc_impl.interface.add_ref();
                        rc_impl
                    };

                    Self {
                        sys,
                        $($field: self.$field.clone()),*
                    }
                }
            }
        };
    }

    pub use impl_clone;
    pub use impl_rc;
    pub use impl_wrap_object;

    #[macro_export]
    macro_rules! define_cef_service {
        (
            #[derive_cef($cef_wrap:ty)]
            $(#[$attr:meta])*
            $pub:vis struct $name:ident {
                $syspub:vis sys: *mut cef::rc::RcImpl<$sys:ty, Self>,
                $($fpub:vis $field:ident: $type:ty,)*
            }
        ) => {
            $(#[$attr])*
            $pub struct $name {
                $syspub sys: *mut cef::rc::RcImpl<$sys, Self>,
                $($fpub $field: $type),*
            }

            $crate::helper::impl_wrap_object!($cef_wrap, $name, $sys);
            $crate::helper::impl_rc!($name);
            $crate::helper::impl_clone!($name, [ $($field),* ]);
        };
    }

    pub use define_cef_service;
}

pub use __macros::*;
