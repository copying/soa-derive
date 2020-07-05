use proc_macro2::{Span, TokenStream};
use syn::{Ident, Visibility};
use quote::{quote, ToTokens};
use rand::{thread_rng, Rng};

pub fn safe_wrap<It>(generated: TokenStream, exported: It, visibility: &Visibility) -> TokenStream
where
    It: Iterator,
    <It as Iterator>::Item: ToTokens,
{
    let mut rng = thread_rng();
    let unique : u32 = rng.gen();
    let module = Ident::new(&format!("__internal_{}", unique), Span::call_site());
    quote!{
        mod #module {
            #generated
        }
        #visibility use #module::{#(#exported),*};
    }
}
