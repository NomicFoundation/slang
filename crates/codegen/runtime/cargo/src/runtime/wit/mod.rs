pub mod cst;
pub mod cursor;
pub mod diagnostic;
pub mod kinds;
pub mod language;
pub mod parse_error;
pub mod parse_output;
pub mod query;
pub mod text_index;

#[allow(
    unused_imports,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_ptr_alignment,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::items_after_statements,
    clippy::match_bool,
    clippy::ptr_as_ptr,
    clippy::single_match_else,
    clippy::uninlined_format_args,
    clippy::too_many_lines,
    clippy::unnecessary_cast,
    clippy::wrong_self_convention
)]
#[path = "generated/generated/bindings.rs"]
mod bindings;

trait IntoFFI<F> {
    fn _into_ffi(self) -> F;
}

trait FromFFI<R> {
    fn _from_ffi(self) -> R;
}

macro_rules! define_wrapper {
    ($name:ident $impl:tt) => {
        paste::paste! {
            #[repr(transparent)]
            pub struct [<$name Wrapper>] (rust::$name);

            impl $crate::wit::IntoFFI<ffi::$name> for rust::$name {
                #[inline]
                fn _into_ffi(self) -> ffi::$name {
                    ffi::$name::new([<$name Wrapper>](self))
                }
            }

            impl $crate::wit::FromFFI<rust::$name> for ffi::$name {
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

            impl $crate::wit::IntoFFI<ffi::$name> for std::rc::Rc<rust::$name> {
                #[inline]
                fn _into_ffi(self) -> ffi::$name {
                    ffi::$name::new([<$name Wrapper>](self))
                }
            }

            impl $crate::wit::FromFFI<std::rc::Rc<rust::$name>> for ffi::$name {
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

            impl $crate::wit::IntoFFI<ffi::$name> for rust::$name {
                #[inline]
                fn _into_ffi(self) -> ffi::$name {
                    ffi::$name::new([<$name Wrapper>](std::cell::RefCell::new(self)))
                }
            }

            impl $crate::wit::FromFFI<rust::$name> for ffi::$name {
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
        impl $crate::wit::IntoFFI<ffi::$name> for rust::$name {
            #[inline]
            fn _into_ffi(self) -> ffi::$name {
                unsafe { core::mem::transmute(self) }
            }
        }

        impl $crate::wit::FromFFI<rust::$name> for ffi::$name {
            #[inline]
            fn _from_ffi(self) -> rust::$name {
                unsafe { core::mem::transmute(self) }
            }
        }
    };
}

// The trick: https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
pub(crate) use {define_rc_wrapper, define_refcell_wrapper, define_wrapper, enum_to_enum};
