use proc_macro2::{Span, TokenStream};
use syn::Ident;
use quote::TokenStreamExt;
use quote::quote;

use crate::utils::safe_wrap;
use crate::input::Input;


pub fn derive(input: &Input) -> TokenStream {
    let name = &input.name;
    let vec_name_str = format!("Vec<{}>", name);
    let other_derive = &input.derive();
    let visibility = &input.visibility;
    let vec_name = &input.vec_name();
    let slice_name = &input.slice_name();
    let slice_mut_name = &input.slice_mut_name();
    let ref_name = &input.ref_name();
    let ptr_name = &input.ptr_name();
    let ptr_mut_name = &input.ptr_mut_name();

    let fields_names = input.fields.iter()
                                   .map(|field| field.ident.clone().unwrap())
                                   .collect::<Vec<_>>();
    let fields_names_1 = &fields_names;
    let fields_names_2 = &fields_names;

    let fields_doc = fields_names.iter()
                                 .map(|field| format!("A vector of `{0}` from a [`{1}`](struct.{1}.html)", field, name))
                                 .collect::<Vec<_>>();

    let fields_types = &input.fields.iter()
                                    .map(|field| &field.ty)
                                    .collect::<Vec<_>>();


    let mut generated = quote! {
        use crate alloc;
        use alloc::raw_vec::RawVec;
        use super::{#slice_name, #slice_mut_name, #ref_name, #ptr_name, #ptr_mut_name};

        /// An analog to `
        #[doc = #vec_name_str]
        /// ` with Struct of Array (SoA) layout
        #[allow(dead_code)]
        #other_derive
        pub struct #vec_name {
            #(
                #[doc = #fields_doc]
                pub #fields_names_1: RawVec<#fields_types>,
            )*
        }

        #[allow(dead_code)]
        impl #vec_name {
            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::new()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new)
            pub fn new() -> #vec_name {
                #vec_name {
                    #(#fields_names_1 : RawVec::new(),)*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::with_capacity()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity),
            /// initializing all fields with the given `capacity`.
            pub fn with_capacity(capacity: usize) -> #vec_name {
                #vec_name {
                    #(#fields_names_1 : RawVec::with_capacity(capacity),)*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::capacity()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.capacity),
            /// the capacity of all fields should be the same.
            pub fn capacity(&self) -> usize {
                match vec![#(self.#fields_names_1.cap,)*].iter().min() {
                    None => usize::MAX, // If there are no fields, capacity is the maximum possible (no need to allocate anything).
                    Some(result) => result
                }
            }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::shrink_to_fit()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.shrink_to_fit)
            // /// shrinking all fields.
            // pub fn shrink_to_fit(&mut self) {
            //     #(self.#fields_names_1.shrink_to_fit(self.len);)*
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::truncate()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.truncate)
            // /// truncating all fields.
            // pub fn truncate(&mut self, len: usize) {
            //     unsafe {
            //         // drop any extra elements
            //         while len < self.len {
            //             // decrement len before the drop_in_place(), so a panic on Drop
            //             // doesn't re-drop the just-failed value.
            //             self.len -= 1;
            //             let i = self.len;
            //
            //             // Drop elements manually
            //             #(ptr::drop_in_place(self.#fields_names_1.ptr().offset(i as isize));)*
            //         }
            //     }
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::push()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push).
            // pub fn push(&mut self, value: #name) {
            //     fn internal_push<T>(buf: &mut RawVec<T>, value: T, index: usize) {
            //         if index == buf.cap() {
            //             buf.double();
            //         }
            //         unsafe {
            //             let ptr = buf.ptr().offset(index as isize);
            //             ptr::write(ptr, value);
            //         }
            //     }
            //
            //     let #name{#(#fields_names_1),*} = value;
            //     let i = self.len;
            //     #(internal_push(self.#fields_names_1, #fields_names_2, i);)*
            //     self.len += 1;
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::is_empty()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty),
            // /// all the fields should have the same length.
            // pub fn is_empty(&self) -> bool {
            //     self.len() == 0
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::swap_remove()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap_remove).
            // pub fn swap_remove(&mut self, index: usize) -> #name {
            //     let length = self.len();
            //     let slices = self.as_mut_slice();
            //     #(slices.#fields_names_1.swap(index, length - 1);)*
            //     self.pop().unwrap()
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::len()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len),
            // /// all the fields should have the same length.
            // pub fn len(&self) -> usize {
            //     self.len
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::insert()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert).
            // pub fn insert(&mut self, index: usize, element: #name) {
            //     fn internal_push<T>(buf: &mut RawVec<T>, len: usize, value: T, index: usize) {
            //         if len == buf.cap() {
            //             buf.double();
            //         }
            //
            //         unsafe {
            //             // infallible
            //             // The spot to put the new value
            //             let p = buf.ptr().offset(index as isize);
            //             // Shift everything over to make space. (Duplicating the
            //             // `index`th element into two consecutive places.)
            //             ptr::copy(p, p.offset(1), len - index);
            //             // Write it in, overwriting the first copy of the `index`th
            //             // element.
            //             ptr::write(p, element);
            //         }
            //     }
            //
            //     let #name{#(#fields_names_1),*} = element;
            //     let len = self.len();
            //     assert!(index <= len);
            //
            //     #(internal_push(self.#fields_names_1, len, #fields_names_2, index);)*
            //     self.len += 1;
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::remove()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove).
            // pub fn remove(&mut self, index: usize) -> #name {
            //     let len = self.len();
            //     assert!(index < len);
            //     fn internal_remove<T>(buf: &mut RawVec<T>, index: usize) {
            //         unsafe {
            //             // infallible
            //             let ret;
            //             {
            //                 // the place we are taking from.
            //                 let ptr = buf.ptr().offset(index as isize);
            //                 // copy it out, unsafely having a copy of the value on
            //                 // the stack and in the vector at the same time.
            //                 ret = ptr::read(ptr);
            //
            //                 // Shift everything down to fill in that spot.
            //                 ptr::copy(ptr.offset(1), ptr, len - index - 1);
            //             }
            //             self.set_len(len - 1);
            //             ret
            //         }
            //     }
            //     #name{#(#fields_names_1: internal_remove(&mut self.#fields_names_2, index)),*}
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::pop()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop).
            // pub fn pop(&mut self) -> Option<#name> {
            //     if self.is_empty() {
            //         None
            //     } else {
            //         self._len -= 1;
            //
            //         unsafe {
            //             #(
            //                 let #fields_names_1 = ptr::read(self.#fields_names_2.ptr().offset(self._len));
            //             )*
            //             Some(#name{#(#fields_names_1: #fields_names_2),*})
            //         }
            //     }
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::append()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append).
            // pub fn append(&mut self, other: &mut #vec_name) {
            //     fn append_elements<T>(source: &mut RawVec<T>, source_len: usize, target: &mut RawVec<T>, target_len: usize) {
            //         target.reserve(source_len);
            //         unsafe {
            //             ptr::copy_nonoverlapping(source.ptr(), source.ptr().offset(target_len), source_len)
            //         }
            //     }
            //
            //     let len = self.len();
            //     let other_len = other.len();
            //     #(
            //         append_elements(self.#fields_names_1, len, &mut other.#fields_names_2, other_len);
            //     )*
            //     self.len += other_len;
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::clear()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear).
            // pub fn clear(&mut self) {
            //     self.truncate(0);
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::as_slice()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_slice).
            // pub fn as_slice(&self) -> #slice_name {
            //     unsafe {
            //         #slice_name {
            //             #(#fields_names_1 : slice::from_raw_parts(self.#fields_names_2.ptr(), self.len()), )*
            //         }
            //     }
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::as_mut_slice()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_slice).
            // pub fn as_mut_slice(&mut self) -> #slice_mut_name {
            //     unsafe {
            //         #slice_name {
            //             #(#fields_names_1 : slice::from_raw_parts_mut(self.#fields_names_2.ptr(), self.len()), )*
            //         }
            //     }
            // }
            //
            // /// Create a slice of this vector matching the given `range`. This
            // /// is analogous to `Index<Range<usize>>`.
            // pub fn slice(&self, range: ::std::ops::Range<usize>) -> #slice_name {
            //     unsafe {
            //         #slice_name {
            //             #(#fields_names_1 : slice::from_raw_parts(self.#fields_names_2.ptr().offset(range.start), range.end - range.start), )*
            //         }
            //     }
            // }
            //
            // /// Create a mutable slice of this vector matching the given
            // /// `range`. This is analogous to `IndexMut<Range<usize>>`.
            // pub fn slice_mut(&mut self, range: ::std::ops::Range<usize>) -> #slice_mut_name {
            //     unsafe {
            //         #slice_name {
            //             #(#fields_names_1 : slice::from_raw_parts_mut(self.#fields_names_2.ptr().offset(range.start), range.end - range.start), )*
            //         }
            //     }
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::as_ptr()`](https://doc.rust-lang.org/std/struct.Vec.html#method.as_ptr).
            // pub fn as_ptr(&self) -> #ptr_name {
            //     #ptr_name {
            //         #(#fields_names_1: self.#fields_names_2.ptr(),)*
            //     }
            // }
            //
            // /// Similar to [`
            // #[doc = #vec_name_str]
            // /// ::as_mut_ptr()`](https://doc.rust-lang.org/std/struct.Vec.html#method.as_mut_ptr).
            // pub fn as_mut_ptr(&mut self) -> #ptr_mut_name {
            //     #ptr_mut_name {
            //         #(#fields_names_1: self.#fields_names_2.ptr(),)*
            //     }
            // }

            pub fn extend_with(n: usize, value: #name) {
                fn internal_extend<T>(buf: &mut RawVec<T>, len: usize, n: usize, value: T) {
                    unsafe {
                        let mut ptr = buf.ptr().offset(len as isize);
                        for _ in 1..n {
                            ptr::write(ptr, value.clone());
                            ptr = ptr.offset(1);
                        }

                        if n > 0 {
                            ptr::write(ptr, value);
                        }
                    }
                }

                if n == 0 {
                    return;
                }

                self.reserve(len);
                #(internal_extend(&mut self.#fields_names_1, self.len(), n, value.#fields_names_2);)*
                self.len += n;
            }
        }
    };

    if input.derives.contains(&Ident::new("Clone", Span::call_site())) {
        generated.append_all(quote!{
            #[allow(dead_code)]
            impl #vec_name {
                /// Similar to [`
                #[doc = #vec_name_str]
                /// ::resize()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.resize).
                pub fn resize(&mut self, new_len: usize, value: #name) {
                    if new_len > len {
                        self.extend_with(new_len - len, value)
                    } else {
                        self.truncate(new_len);
                    }
                }
            }
        });
    }

    safe_wrap(generated, [&vec_name].iter(), visibility)

}