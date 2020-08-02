use proc_macro2::{Span, TokenStream};
use syn::Ident;
use quote::TokenStreamExt;
use quote::quote;

use crate::input::Input;


pub fn derive(input: &Input) -> TokenStream {
    let mut debug = false;
    let mut clone = false;

    let name = &input.name;
    let vec_name_str = format!("Vec<{}>", name);

    let derives : Vec<&Ident> = input.derives.iter()
        .filter(|ident| {
            match ident.to_string().as_str() {
                "Debug" => {
                    debug = true;
                    false
                },
                "Clone" => {
                    clone = true;
                    false
                },
                _ => true
            }
        })
        .collect();
    let other_derive = quote!{
        #[derive(
            #(#derives,)*
        )]
    };
    let visibility = &input.visibility;
    let detail_mod = Ident::new(&format!("__detail_vec_{}", name.to_string().to_lowercase()), Span::call_site());
    let original_name = &input.name;
    let vec_name = &input.vec_name();
    let slice_name = &input.slice_name();
    let slice_mut_name = &input.slice_mut_name();
    let ref_name = &input.ref_name();
    let ref_mut_name = &input.ref_mut_name();
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

    let mut imports = quote!{
        extern crate alloc;

        use alloc::raw_vec::RawVec;
        use std::ptr;
        use std::slice;

        use super::{#original_name, #slice_name, #slice_mut_name, #ref_name, #ref_mut_name, #ptr_name, #ptr_mut_name};
    };
    if debug {
        imports.append_all(quote!{
            use std::fmt;
        });
    }
    let mut generated = quote! {
        #imports

        /// An analog to `
        #[doc = #vec_name_str]
        /// ` with Struct of Array (SoA) layout
        #[allow(dead_code)]
        pub struct #vec_name {
            #(
                #[doc = #fields_doc]
                pub #fields_names_1: RawVec<#fields_types>,
            )*
            pub len: usize,
        }

        #[allow(dead_code)]
        impl #vec_name {
            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::new()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.new)
            pub fn new() -> #vec_name {
                #vec_name {
                    #(#fields_names_1: RawVec::new(),)*
                    len: 0,
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::with_capacity()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity),
            /// initializing all fields with the given `capacity`.
            pub fn with_capacity(capacity: usize) -> #vec_name {
                #vec_name {
                    #(#fields_names_1 : RawVec::with_capacity(capacity),)*
                    len: 0,
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::capacity()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.capacity),
            /// the capacity of all fields should be the same.
            pub fn capacity(&self) -> usize {
                let vec: Vec<usize> = vec![#(self.#fields_names_1.capacity()),*];
                match vec.iter().min() {
                    None => usize::MAX, // If there are no fields, capacity is the maximum possible (no need to allocate anything).
                    Some(result) => *result
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::reserve()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve),
            /// making sure all fields have the required additional space.
            pub fn reserve(&mut self, additional: usize) {
                #(self.#fields_names_1.reserve(self.len, additional);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::reserve_exact()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.reserve_exact)
            /// reserving the same `additional` space for all fields.
            pub fn reserve_exact(&mut self, additional: usize) {
                #(self.#fields_names_1.reserve_exact(self.len, additional);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::shrink_to_fit()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.shrink_to_fit)
            /// shrinking all fields.
            pub fn shrink_to_fit(&mut self) {
                #(self.#fields_names_1.shrink_to_fit(self.len);)*
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::truncate()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.truncate)
            /// truncating all fields.
            pub fn truncate(&mut self, len: usize) {
                unsafe {
                    // drop any extra elements
                    while len < self.len {
                        // decrement len before the drop_in_place(), so a panic on Drop
                        // doesn't re-drop the just-failed value.
                        self.len -= 1;
                        let i = self.len;

                        // Drop elements manually
                        #(ptr::drop_in_place(self.#fields_names_1.ptr().add(i));)*
                    }
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::push()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push).
            pub fn push(&mut self, value: #name) {
                fn internal_push<T>(buf: &mut RawVec<T>, value: T, index: usize) {
                    unsafe {
                        let ptr = buf.ptr().add(index);
                        ptr::write(ptr, value);
                    }
                }

                let #name{#(#fields_names_1),*} = value;
                let i = self.len;
                self.reserve(1);
                #(internal_push(&mut self.#fields_names_1, #fields_names_2, i);)*
                self.len += 1;
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::is_empty()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.is_empty),
            /// all the fields should have the same length.
            pub fn is_empty(&self) -> bool {
                self.len == 0
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::get()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get)
            pub fn get<'a: 'b, 'b>(&'a self, index: usize) -> Option<#ref_name<'b>> {
                let slice = self.as_slice();
                Some(#ref_name {
                    #(
                        #fields_names_1: slice.#fields_names_2.get(index)?,
                    )*
                })
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::get_mut()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.get_mut)
            pub fn get_mut<'a: 'b, 'b>(&'a mut self, index: usize) -> Option<#ref_mut_name<'b>> {
                let slice = self.as_mut_slice();
                Some(#ref_mut_name {
                    #(
                        #fields_names_1: slice.#fields_names_2.get_mut(index)?,
                    )*
                })
            }


            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::swap_remove()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.swap_remove).
            pub fn swap_remove(&mut self, index: usize) -> #name {
                let length = self.len;
                let mut slices = self.as_mut_slice();
                #(slices.#fields_names_1.swap(index, length - 1);)*
                self.pop().unwrap()
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::len()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.len),
            /// all the fields should have the same length.
            pub fn len(&self) -> usize {
                self.len
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::insert()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert).
            pub fn insert(&mut self, index: usize, element: #name) {
                fn internal_push<T>(buf: &mut RawVec<T>, len: usize, value: T, index: usize) {
                    unsafe {
                        // infallible
                        // The spot to put the new value
                        let p = buf.ptr().add(index);
                        // Shift everything over to make space. (Duplicating the
                        // `index`th element into two consecutive places.)
                        ptr::copy(p, p.offset(1), len - index);
                        // Write it in, overwriting the first copy of the `index`th
                        // element.
                        ptr::write(p, value);
                    }
                }

                let #name{#(#fields_names_1),*} = element;
                let len = self.len();
                assert!(index <= len);

                self.reserve(1);
                #(internal_push(&mut self.#fields_names_1, len, #fields_names_2, index);)*
                self.len += 1;
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::remove()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.remove).
            pub fn remove(&mut self, index: usize) -> #name {
                fn internal_remove<T>(buf: &mut RawVec<T>, len: usize, index: usize) -> T {
                    unsafe {
                        // infallible
                        let ret;
                        {
                            // the place we are taking from.
                            let ptr = buf.ptr().add(index);
                            // copy it out, unsafely having a copy of the value on
                            // the stack and in the vector at the same time.
                            ret = ptr::read(ptr);

                            // Shift everything down to fill in that spot.
                            ptr::copy(ptr.offset(1), ptr, len - index - 1);
                        }
                        ret
                    }
                }

                let len = self.len();
                assert!(index < len);
                self.len -= 1;
                #name{#(#fields_names_1: internal_remove(&mut self.#fields_names_2, len, index)),*}
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::pop()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.pop).
            pub fn pop(&mut self) -> Option<#name> {
                if self.is_empty() {
                    None
                } else {
                    self.len -= 1;

                    unsafe {
                        #(
                            let #fields_names_1 = ptr::read(self.#fields_names_2.ptr().offset(self.len as isize));
                        )*
                        Some(#name{#(#fields_names_1: #fields_names_2),*})
                    }
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::append()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.append).
            pub fn append(&mut self, other: &mut #vec_name) {
                fn append_elements<T>(src: &RawVec<T>, srclen: usize, dst: &mut RawVec<T>, dstlen: usize) {
                    dst.reserve(dstlen, srclen);
                    unsafe {
                        ptr::copy_nonoverlapping(src.ptr(), dst.ptr().add(dstlen), srclen)
                    }
                }

                let len = self.len();
                let otherlen = other.len();
                self.reserve(otherlen);
                #(
                    append_elements(&other.#fields_names_1, otherlen, &mut self.#fields_names_2, len);
                )*
                other.len = 0;
                self.len += otherlen;
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::clear()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.clear).
            pub fn clear(&mut self) {
                self.truncate(0);
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::split_off()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.split_off).
            pub fn split_off(&mut self, at: usize) -> Self {
                #[cold]
                #[inline(never)]
                fn assert_failed(at: usize, len: usize) -> ! {
                    panic!("`at` split index (is {}) should be <= len (is {})", at, len);
                }

                if at > self.len() {
                    assert_failed(at, self.len());
                }

                let other_len = self.len - at;
                let mut other = #vec_name::with_capacity(other_len);

                // Unsafely `set_len` and copy items to `other`.
                unsafe {
                    self.len = at;
                    other.len = other_len;

                    #(
                        ptr::copy_nonoverlapping(
                            self.#fields_names_1.ptr().add(at),
                            other.#fields_names_2.ptr(),
                            other_len,
                        );
                    )*
                }
                other
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_slice()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_slice).
            pub fn as_slice(&self) -> #slice_name {
                unsafe {
                    #slice_name {
                        #(#fields_names_1 : slice::from_raw_parts(self.#fields_names_2.ptr(), self.len()), )*
                    }
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_mut_slice()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.as_mut_slice).
            pub fn as_mut_slice(&mut self) -> #slice_mut_name {
                unsafe {
                    #slice_mut_name {
                        #(#fields_names_1 : slice::from_raw_parts_mut(self.#fields_names_2.ptr(), self.len()), )*
                    }
                }
            }

            /// Create a slice of this vector matching the given `range`. This
            /// is analogous to `Index<Range<usize>>`.
            pub fn slice(&self, range: ::std::ops::Range<usize>) -> #slice_name {
                unsafe {
                    #slice_name {
                        #(#fields_names_1 : slice::from_raw_parts(self.#fields_names_2.ptr().offset(range.start as isize), range.end - range.start), )*
                    }
                }
            }

            /// Create a mutable slice of this vector matching the given
            /// `range`. This is analogous to `IndexMut<Range<usize>>`.
            pub fn slice_mut(&mut self, range: ::std::ops::Range<usize>) -> #slice_mut_name {
                unsafe {
                    #slice_mut_name {
                        #(#fields_names_1 : slice::from_raw_parts_mut(self.#fields_names_2.ptr().offset(range.start as isize), range.end - range.start), )*
                    }
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::retain()`](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.retain).
            pub fn retain<F>(&mut self, mut f: F)
                where F: FnMut(#ref_name) -> bool
            {
                let len = self.len();
                let mut del = 0;

                {
                    let mut slice = self.as_mut_slice();
                    for i in 0..len {
                        if !f(slice.get(i).unwrap()) {
                            del += 1;
                        } else if del > 0 {
                            slice.swap(i - del, i);
                        }
                    }
                }
                if del > 0 {
                    self.truncate(len - del);
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_ptr()`](https://doc.rust-lang.org/std/struct.Vec.html#method.as_ptr).
            pub fn as_ptr(&self) -> #ptr_name {
                #ptr_name {
                    #(#fields_names_1: self.#fields_names_2.ptr(),)*
                }
            }

            /// Similar to [`
            #[doc = #vec_name_str]
            /// ::as_mut_ptr()`](https://doc.rust-lang.org/std/struct.Vec.html#method.as_mut_ptr).
            pub fn as_mut_ptr(&mut self) -> #ptr_mut_name {
                #ptr_mut_name {
                    #(#fields_names_1: self.#fields_names_2.ptr(),)*
                }
            }

            pub fn extend_with(&mut self, n: usize, value: #name) {
                fn internal_extend<T: Clone>(buf: &mut RawVec<T>, len: usize, n: usize, value: T) {
                    unsafe {
                        let mut ptr = buf.ptr().add(len);
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

                self.reserve(n);
                #(internal_extend(&mut self.#fields_names_1, self.len, n, value.#fields_names_2);)*
                self.len += n;
            }
        }


        impl Drop for #vec_name {
            fn drop(&mut self) {
                unsafe {
                    let mut slice = self.as_mut_slice();
                    // use drop for [T]
                    #(ptr::drop_in_place(&mut slice.#fields_names_1);)*
                }
                // RawVec handles deallocation
            }
        }
    };

    if clone {
        generated.append_all(quote!{
            impl Clone for #vec_name {
                fn clone(&self) -> #vec_name {
                    fn internal_clone<T>(source: & RawVec<T>, len: usize) -> RawVec<T>{
                        let target = RawVec::with_capacity(len);
                        let source_slice = slice::from_raw_parts(source.ptr(), len);
                        let target_slice = slice::from_raw_parts_mut(target.ptr(), len);
                        for i in 0..len {
                            target[i] = source[i];
                        }
                        target
                    }

                    #vec_name {
                        #(#fields_names_1: internal_clone(&self.#fields_names_2, self.len),)*
                        len: self.len,
                    }
                }
            }
        });
    }

    if debug {
        generated.append_all(quote!{
            impl fmt::Debug for #vec_name {
                fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                    f.debug_struct(stringify!(#vec_name))
                        #(
                            .field(stringify!(#fields_names_1), &(
                                f.debug_list()
                                    .finish()
                            ))
                        )*
                        .finish()
                }
            }
        });
    }

    quote!{
        #[allow(non_snake_case, dead_code)]
        mod #detail_mod {
            #generated
        }
        pub use #detail_mod::#vec_name;
    }
}
