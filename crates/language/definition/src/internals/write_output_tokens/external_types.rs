use std::path::PathBuf;
use std::rc::Rc;

use indexmap::{IndexMap, IndexSet};
use infra_utils::paths::PathExtensions;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use semver::Version;

use crate::internals::WriteOutputTokens;

impl WriteOutputTokens for bool {
    fn write_output_tokens(&self) -> TokenStream {
        let value = format_ident!("{self}");

        quote! {
            #value
        }
    }
}

impl<T: WriteOutputTokens> WriteOutputTokens for Box<T> {
    fn write_output_tokens(&self) -> TokenStream {
        let value = self.as_ref().write_output_tokens();

        quote! {
            #value.into()
        }
    }
}

impl WriteOutputTokens for char {
    fn write_output_tokens(&self) -> TokenStream {
        let value = Literal::character(*self);

        quote! {
            #value
        }
    }
}

impl<K: WriteOutputTokens, V: WriteOutputTokens> WriteOutputTokens for IndexMap<K, V> {
    fn write_output_tokens(&self) -> TokenStream {
        let keys = self.keys().map(K::write_output_tokens);
        let values = self.values().map(V::write_output_tokens);

        quote! {
            [ #( (#keys, #values) ),* ].into()
        }
    }
}

impl<T: WriteOutputTokens> WriteOutputTokens for IndexSet<T> {
    fn write_output_tokens(&self) -> TokenStream {
        let items = self.iter().map(T::write_output_tokens);

        quote! {
            [ #( #items ),* ].into()
        }
    }
}

impl<T: WriteOutputTokens> WriteOutputTokens for Option<T> {
    fn write_output_tokens(&self) -> TokenStream {
        match self {
            Some(value) => {
                let value = value.write_output_tokens();

                quote! {
                    Some(#value)
                }
            }
            None => {
                quote! {
                    None
                }
            }
        }
    }
}

impl WriteOutputTokens for PathBuf {
    fn write_output_tokens(&self) -> TokenStream {
        let value = Literal::string(self.unwrap_str());

        quote! {
            #value.into()
        }
    }
}

impl<T: WriteOutputTokens> WriteOutputTokens for Rc<T> {
    fn write_output_tokens(&self) -> TokenStream {
        let value = self.as_ref().write_output_tokens();

        quote! {
            #value.into()
        }
    }
}

impl WriteOutputTokens for String {
    fn write_output_tokens(&self) -> TokenStream {
        let value = Literal::string(self.as_str());

        quote! {
            #value.into()
        }
    }
}

impl WriteOutputTokens for usize {
    fn write_output_tokens(&self) -> TokenStream {
        let value = Literal::usize_suffixed(*self);

        quote! {
            #value.into()
        }
    }
}

impl WriteOutputTokens for u8 {
    fn write_output_tokens(&self) -> TokenStream {
        let value = Literal::u8_suffixed(*self);

        quote! {
            #value.into()
        }
    }
}

impl<T: WriteOutputTokens> WriteOutputTokens for Vec<T> {
    fn write_output_tokens(&self) -> TokenStream {
        let items = self.iter().map(T::write_output_tokens);

        quote! {
            [ #( #items ),* ].into()
        }
    }
}

impl WriteOutputTokens for Version {
    fn write_output_tokens(&self) -> TokenStream {
        let major = Literal::u64_unsuffixed(self.major);
        let minor = Literal::u64_unsuffixed(self.minor);
        let patch = Literal::u64_unsuffixed(self.patch);

        quote! {
            semver::Version::new(#major, #minor, #patch)
        }
    }
}
