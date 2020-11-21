extern crate proc_macro;

use quote::quote;
use syn;

use proc_macro::TokenStream;

#[proc_macro_derive(ShapeAccessors)]
pub fn shape_accessors_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    let gen = quote! {
        impl Shape for #name {
            fn id(&self) -> u32 {
                self.id
            }

            fn transformation(&self) -> &Matrix {
                &self.transformation
            }

            fn transformation_mut(&mut self) -> &mut Matrix {
                &mut self.transformation
            }

            fn material(&self) -> &Material {
                &self.material
            }

            fn material_mut(&mut self) -> &mut Material {
                &mut self.material
            }
        }
    };

    gen.into()
}
