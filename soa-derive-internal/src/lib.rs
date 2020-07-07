#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::needless_return, clippy::redundant_field_names)]
#![allow(clippy::use_self, clippy::too_many_lines)]
// TODO: improve the code and make it simpler to read
#![allow(clippy::cognitive_complexity)]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::TokenStreamExt;

mod input;
mod vec;
mod single_length_vec;
mod refs;
mod ptr;
mod slice;
mod iter;
mod single_length_iter;

const USE_RAW_VEC : bool = cfg!(feature = "use_raw_vec");

#[proc_macro_derive(StructOfArray, attributes(soa_derive))]
pub fn soa_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    let input = input::Input::new(ast);

    let mut generated = TokenStream::new();
    let vec = if USE_RAW_VEC {
        single_length_vec::derive(&input)
    } else {
        vec::derive(&input)
    };
    generated.append_all(vec);
    generated.append_all(refs::derive(&input));
    generated.append_all(ptr::derive(&input));
    generated.append_all(slice::derive(&input));
    generated.append_all(slice::derive_mut(&input));
    let iter = if USE_RAW_VEC {
        single_length_iter::derive(&input)
    } else {
        iter::derive(&input)
    };
    generated.append_all(iter);
    // println!("{}", generated);
    generated.into()
}
