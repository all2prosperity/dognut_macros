#![feature(proc_macro_quote)]
// use proc_macro2::TokenStream;
// use syn::spanned::Spanned;
// use syn::*;
// use quote::*;
// use quote::ToTokens;
use syn::spanned::Spanned;
use syn::*;


#[proc_macro_derive(TriangularInverse)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let st = parse_macro_input!(input as DeriveInput);
    let struct_name_literal = st.ident.to_string();
    let struct_name_indent = syn::Ident::new(&struct_name_literal, st.span());
    let mut ret = impl_first(&struct_name_indent);

    for i in 2..17 {
        ret.extend(impl_once(&struct_name_indent, &st, i))
    }
    
    ret.into()
}

fn impl_first(struct_name: &syn::Ident) -> proc_macro2::TokenStream {
    quote::quote!(
        impl #struct_name<1, 1> {
            pub fn upper_triangular_matrix_inverse(&self) -> Self{
                Self::from_vec(vec![1. / self.index(0, 0)])
            }
        }
    )
}

fn impl_once(struct_name: &syn::Ident, st: &syn::DeriveInput, times: u8) -> proc_macro2::TokenStream {
    let num_ident = syn::LitInt::new(&times.to_string(), st.span());
    let small = times / 2;
    let big = times - small;
    let small_ident = syn::LitInt::new(&small.to_string(), st.span());
    let big_ident = syn::LitInt::new(&big.to_string(), st.span());
    quote::quote!(
        impl #struct_name<#num_ident, #num_ident> {
            pub fn upper_triangular_matrix_inverse(&self) -> Self{
                let left_up = self.cut::<#small_ident, #small_ident>(0, 0);
                let right_down = self.cut::<#big_ident, #big_ident>(#small_ident, #small_ident);
                let right_up = self.cut::<#small_ident, #big_ident>(0, #small_ident);

                let left_up_1 = left_up.upper_triangular_matrix_inverse();
                let right_down_1 = right_down.upper_triangular_matrix_inverse();
                let mut right_up_new = &(&left_up_1 * &right_up) * &right_down_1;
                right_up_new.mul_assign(-1.);

                let mut ret = Self::new();
                ret.paste::<#small_ident, #small_ident>(&left_up_1, 0, 0);
                ret.paste::<#big_ident, #big_ident>(&right_down_1, #small_ident, #small_ident);
                ret.paste::<#small_ident, #big_ident>(&right_up_new, 0, #small_ident);
                ret
            }

            pub fn inverse_matrix(&self) -> Option<Self>{
                if let Some((l, l_1, u)) = self.l_u_split() {
                    let u_1 = u.upper_triangular_matrix_inverse();
                    Some(u_1 * l_1)
                }
                else {
                    None
                }
            }
        }
    )
}

