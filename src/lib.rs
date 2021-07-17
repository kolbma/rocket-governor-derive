//! [rocket_governor_derive] is a [rocket] guard implementation of the
//! [governor] rate limiter.  
//!
//! It is used in combination with [rocket_governor].
//!
//! [governor]: https://docs.rs/governor
//! [rocket]: https://docs.rs/rocket/
//! [rocket_governor]: https://docs.rs/rocket_governor/
//! [rocket_governor_derive]: https://docs.rs/rocket_governor_derive/

#![deny(unsafe_code)]
#![deny(warnings)]
#![deny(clippy::all)]

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::DeriveInput;

/// Derive to implement RocketGovernor guard
///
/// [rocket_governor_derive] is a [rocket] guard implementation of the
/// [governor] rate limiter.  
///
/// It is used in combination with [rocket_governor].
///
/// Declare a struct with `#[derive(RocketGovernor)]` and implement the
/// missing `quota()` method.
///
/// [governor]: https://docs.rs/governor
/// [rocket]: https://docs.rs/rocket/
/// [rocket_governor]: https://docs.rs/rocket_governor/
/// [rocket_governor_derive]: https://docs.rs/rocket_governor_derive/
///
#[proc_macro_derive(RocketGovernor)]
pub fn derive_rocket_governor_fn(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident;
    let catcher_method = &format_ident!(
        "{}_rocket_governor_catcher",
        struct_name.to_string().to_lowercase()
    );

    let gen = quote! {
        #[::rocket::async_trait]
        impl<'r> ::rocket::request::FromRequest<'r> for #struct_name {
            type Error = ::rocket_governor::LimitError;

            /// Caller for implementation in 
            /// [rocket_governor::RocketGovernorMacroUtil](https://docs.rs/rocket-governor/latest/rocket_governor/struct.RocketGovernorMacroUtil.html)
            async fn from_request(
                request: &'r ::rocket::Request<'_>,
            ) -> ::rocket::request::Outcome<Self, ::rocket_governor::LimitError> {
                ::rocket_governor::RocketGovernorMacroUtil::<'r, #struct_name>::handle_from_request(request)
            }
        }

        impl Default for #struct_name {
            fn default() -> Self {
                Self {}
            }
        }

        #[::rocket::catch(429)]
        pub fn #catcher_method<'c>(request: &'c ::rocket::Request) -> &'c ::rocket_governor::LimitError {
            #struct_name::rocket_governor_catcher(request)
        }
    };
    gen.into()
}

/// Derive to implement RocketGovernor guard with handling struct members
///
/// [rocket_governor_derive] is a [rocket] guard implementation of the
/// [governor] rate limiter.  
///
/// It is used in combination with [rocket_governor].
///
/// Declare a struct with `#[derive(RocketGovernorWithMember)]` and implement the
/// missing `quota()` method and the [Default] trait.
///
/// [Default]: https://doc.rust-lang.org/std/default/trait.Default.html
/// [governor]: https://docs.rs/governor
/// [rocket]: https://docs.rs/rocket/
/// [rocket_governor]: https://docs.rs/rocket_governor/
/// [rocket_governor_derive]: https://docs.rs/rocket_governor_derive/
///
#[proc_macro_derive(RocketGovernorWithMember)]
pub fn derive_rocket_governor_with_member_fn(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();

    let struct_name = &ast.ident;
    let catcher_method = &format_ident!(
        "{}_rocket_governor_catcher",
        struct_name.to_string().to_lowercase()
    );

    let gen = quote! {
        #[::rocket::async_trait]
        impl<'r> ::rocket::request::FromRequest<'r> for #struct_name {
            type Error = ::rocket_governor::LimitError;

            /// Caller for implementation in 
            /// [rocket_governor::RocketGovernorMacroUtil](https://docs.rs/rocket-governor/latest/rocket_governor/struct.RocketGovernorMacroUtil.html)
            async fn from_request(
                request: &'r ::rocket::Request<'_>,
            ) -> ::rocket::request::Outcome<Self, ::rocket_governor::LimitError> {
                ::rocket_governor::RocketGovernorMacroUtil::<'r, #struct_name>::handle_from_request(request)
            }
        }

        #[::rocket::catch(429)]
        pub fn #catcher_method<'c>(request: &'c ::rocket::Request) -> &'c ::rocket_governor::LimitError {
            #struct_name::rocket_governor_catcher(request)
        }
    };
    gen.into()
}
