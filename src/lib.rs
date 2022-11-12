//! # Persitent Structs
//!
//! A small derive Macro for structs, that generates a `with_<name>` and `update_<name>`
//! method for each field of a struct, e.g:
//!
//! ```rust
//! use persistent_structs::PersistentStruct;
//!
//! #[derive(PersistentStruct, PartialEq)]
//! struct Foo {
//!     pub foo: u8,
//! }
//!
//! fn main() {
//!     let foo = Foo { foo: 1 };
//!     let foo = foo.with_foo(5);
//!     assert!(foo == Foo { foo: 5 });
//!
//!     let foo = foo.update_foo(|x| x + 1);
//!     assert!(foo.foo == 6);
//! }
//! ```

use proc_macro::TokenStream;
use quote::format_ident;
use syn::{parse_macro_input, DeriveInput, Field, ItemFn};

#[proc_macro_derive(PersistentStruct)]
pub fn derive_persistent_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let syn::Data::Struct(data) = &input.data else {
        panic!("PersistentStruct may only be used with structs");
    };

    let syn::Fields::Named(named_fields) = &data.fields else {
        panic!("PersistentStruct may only be Used on structs with named fields");
    };

    let set_functions = named_fields.named.iter().map(|f: &Field| -> ItemFn {
        let vis = &f.vis;
        let field_name = f.ident.as_ref().expect("Fields must have a name");
        let ident = format_ident!("with_{}", field_name);
        let ty = &f.ty;

        syn::parse_quote! {
            #vis fn #ident(self, x: #ty) -> Self {
                Self {
                    #field_name: x,
                    ..self
                }
            }
        }
    });

    let update_functions = named_fields.named.iter().map(|f: &Field| -> ItemFn {
        let vis = &f.vis;
        let field_name = f.ident.as_ref().expect("Fields must have a name");
        let ident = format_ident!("update_{}", field_name);
        let ty = &f.ty;

        syn::parse_quote! {
            #vis fn #ident(self, f: impl FnOnce(#ty) -> #ty) -> Self {
                Self {
                    #field_name: f(self.#field_name),
                    ..self
                }
            }
        }
    });

    let name = &input.ident;
    let generics = &input.generics;

    quote::quote! {
        impl #generics #name #generics {
            #(#set_functions)*
            #(#update_functions)*
        }
    }
    .into()
}
