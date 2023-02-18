//! Simple wrapper generator around bytemuck pod types.
//!
//! Sometimes, you want to expose raw byte bytemuck conversions, but not actually publicly depend
//! on bytemuck. This crate's macro is for that.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Wrapmuck)]
pub fn derive_wrapmuck(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let DeriveInput { ident, .. } = input;

    let expanded = quote! {
        impl #ident {
            /// Create a new zeroed value.
            pub fn new() -> Self {
                let value = Self(bytemuck::Zeroable::zeroed());
                value
            }

            /// Convert a byte slice to a value reference.
            pub fn from_bytes(bytes: &[u8]) -> &Self {
                bytemuck::TransparentWrapper::wrap_ref(bytemuck::from_bytes(bytes))
            }

            /// Convert a mutable byte slice to a mutable value reference.
            pub fn from_bytes_mut(bytes: &mut [u8]) -> &mut Self {
                bytemuck::TransparentWrapper::wrap_mut(bytemuck::from_bytes_mut(bytes))
            }

            /// Get the inner data as bytes.
            pub fn as_bytes(&self) -> &[u8] {
                bytemuck::bytes_of(&self.0)
            }

            /// Get the inner data as mutable bytes.
            pub fn as_bytes_mut(&mut self) -> &mut [u8] {
                bytemuck::bytes_of_mut(&mut self.0)
            }
        }
    };

    TokenStream::from(expanded)
}
