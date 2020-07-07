use proc_macro2::{Span, TokenStream};
use syn::{Ident, Visibility};
use quote::TokenStreamExt;
use quote::quote;

use crate::input::Input;

pub fn derive(input: &Input) -> TokenStream {
    let name = &input.name;
    let visibility = &input.visibility;
    let detail_mod = Ident::new(&format!("__detail_iter_{}", name.to_string().to_lowercase()), Span::call_site());
    let vec_name = &input.vec_name();
    let slice_name = &input.slice_name();
    let slice_mut_name = &input.slice_mut_name();
    let ref_name = &input.ref_name();
    let ref_mut_name = &input.ref_mut_name();

    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;

    quote!{
        mod #detail_mod {
            use super::{#vec_name, #ref_name, #ref_mut_name};

            struct VecIter<'a> {
                vec: &'a #vec_name,
                n: usize,
            }

            impl<'a> Iterator for VecIter<'a> {
                type Item = #ref_name<'a>;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.n >= self.vec.len() {
                        return None;
                    }

                    let item = (&self.vec).get(self.n);
                    self.n += 1;
                    item
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    if self.n >= self.vec.len() {
                        return (0, Some(0))
                    }
                    let left = self.vec.len() - self.n;
                    (left, Some(left))
                }
            }

            impl #vec_name {
                fn iter<'a>(&'a self) -> VecIter<'a> {
                    VecIter {
                        vec: self,
                        n: 0,
                    }
                }
            }


            struct VecIterMut<'a> {
                vec: &'a mut #vec_name,
                n: usize,
            }

            impl<'a> Iterator for VecIterMut<'a> {
                type Item = #ref_mut_name<'a>;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.n >= self.vec.len() {
                        return None;
                    }
                    let n = self.n;
                    unsafe {
                        Some(#ref_mut_name {
                            #(
                                #fields_names_1: &mut *self.vec.#fields_names_2.ptr().offset(n as isize),
                            )*
                        })
                    }
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    if self.n >= self.vec.len() {
                        return (0, Some(0))
                    }
                    let left = self.vec.len() - self.n;
                    (left, Some(left))
                }
            }

            impl #vec_name {
                fn iter_mut<'a>(&'a mut self) -> VecIterMut<'a> {
                    VecIterMut {
                        vec: self,
                        n: 0,
                    }
                }
            }
        }
    }
}
