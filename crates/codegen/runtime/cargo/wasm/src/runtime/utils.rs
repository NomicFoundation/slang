pub trait IntoFFI<F> {
    fn _into_ffi(self) -> F;
}

pub trait FromFFI<R> {
    fn _from_ffi(self) -> R;
}

macro_rules! define_wrapper {
    ($name:ident $impl:tt) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct [<$name Wrapper>] (rust::$name);

            impl $crate::wasm_crate::utils::IntoFFI<ffi::$name> for rust::$name {
                #[inline]
                fn _into_ffi(self) -> ffi::$name {
                    ffi::$name::new([<$name Wrapper>](self))
                }
            }

            impl $crate::wasm_crate::utils::FromFFI<rust::$name> for ffi::$name {
                #[inline]
                fn _from_ffi(self) -> rust::$name {
                    self.into_inner::<[<$name Wrapper>]>().0
                }
            }

            // As owned argument
            impl ffi:: $name {
                #[inline]
                pub fn _borrow_ffi(&self) -> &rust::$name {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
            }

            // As borrowed argument
            impl<'a> ffi:: [<$name Borrow>] <'a> {
                #[inline]
                pub fn _borrow_ffi(&'a self) -> &'a rust::$name {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
            }

            // As self
            impl [<$name Wrapper>] {
                #[inline]
                pub fn _borrow_ffi(&self) -> &rust::$name {
                    &self.0
                }
            }

            impl ffi:: [<Guest $name>] for [<$name Wrapper>] $impl
        }
    };
}

macro_rules! define_rc_wrapper {
    ($name:ident $impl:tt) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct [<$name Wrapper>] (std::rc::Rc<rust::$name>);

            impl $crate::wasm_crate::utils::IntoFFI<ffi::$name> for std::rc::Rc<rust::$name> {
                #[inline]
                fn _into_ffi(self) -> ffi::$name {
                    ffi::$name::new([<$name Wrapper>](self))
                }
            }

            impl $crate::wasm_crate::utils::FromFFI<std::rc::Rc<rust::$name>> for ffi::$name {
                #[inline]
                fn _from_ffi(self) -> std::rc::Rc<rust::$name> {
                    self.into_inner::<[<$name Wrapper>]>().0
                }
            }

            // As owned argument
            impl ffi:: $name {
                #[inline]
                pub fn _borrow_ffi(&self) -> &std::rc::Rc<rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
            }

            // As borrowed argument
            impl<'a> ffi:: [<$name Borrow>] <'a> {
                #[inline]
                pub fn _borrow_ffi(&self) -> &std::rc::Rc<rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
            }

            // As self
            impl [<$name Wrapper>] {
                #[inline]
                pub fn _borrow_ffi(&self) -> &std::rc::Rc<rust::$name> {
                    &self.0
                }
            }

            impl ffi:: [<Guest $name>] for [<$name Wrapper>] $impl
        }
    };
}

macro_rules! define_refcell_wrapper {
    ($name:ident $impl:tt) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct [<$name Wrapper>] (std::cell::RefCell<rust::$name>);

            impl $crate::wasm_crate::utils::IntoFFI<ffi::$name> for rust::$name {
                #[inline]
                fn _into_ffi(self) -> ffi::$name {
                    ffi::$name::new([<$name Wrapper>](std::cell::RefCell::new(self)))
                }
            }

            impl $crate::wasm_crate::utils::FromFFI<rust::$name> for ffi::$name {
                #[inline]
                fn _from_ffi(self) -> rust::$name {
                    self.into_inner::<[<$name Wrapper>]>().0.into_inner()
                }
            }

            // As owned argument
            impl ffi:: $name {
                #[inline]
                pub fn _borrow_ffi(&self) -> std::cell::Ref<'_, rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
                #[inline]
                pub fn _borrow_mut_ffi(&self) -> std::cell::RefMut<'_, rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_mut_ffi()
                }
            }

            // As borrowed argument
            impl<'a> ffi:: [<$name Borrow>] <'a> {
                #[inline]
                pub fn _borrow_ffi(&self) -> std::cell::Ref<'_, rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_ffi()
                }
                #[inline]
                pub fn _borrow_mut_ffi(&self) -> std::cell::RefMut<'_, rust::$name> {
                    self.get::<[<$name Wrapper>]>()._borrow_mut_ffi()
                }
            }

            // As self
            impl [<$name Wrapper>] {
                #[inline]
                pub fn _borrow_ffi(&self) -> std::cell::Ref<'_, rust::$name> {
                    self.0.borrow()
                }
                #[inline]
                pub fn _borrow_mut_ffi(&self) -> std::cell::RefMut<'_, rust::$name> {
                    self.0.borrow_mut()
                }
            }

            impl ffi:: [<Guest $name>] for [<$name Wrapper>] $impl
        }
    };
}

macro_rules! enum_to_enum {
    ($name:ident) => {
        impl $crate::wasm_crate::utils::IntoFFI<ffi::$name> for rust::$name {
            #[inline]
            fn _into_ffi(self) -> ffi::$name {
                unsafe { core::mem::transmute(self) }
            }
        }

        impl $crate::wasm_crate::utils::FromFFI<rust::$name> for ffi::$name {
            #[inline]
            fn _from_ffi(self) -> rust::$name {
                unsafe { core::mem::transmute(self) }
            }
        }
    };
}

pub(crate) use {define_rc_wrapper, define_refcell_wrapper, define_wrapper, enum_to_enum};
