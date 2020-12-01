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

            // It could be implemented the code to return Option<Arc<dyn Shape>>, however, it would
            // then require the mutable version, in order to change the reference when required.
            //
            fn parent(&self) -> &Mutex<Weak<dyn Shape>> {
                &self.parent
            }

            fn children(&self) -> &Mutex<Vec<Arc<dyn Shape>>> {
                &self.children
            }

            fn transform(&self) -> &Matrix {
                &self.transform
            }

            fn transform_mut(&mut self) -> &mut Matrix {
                &mut self.transform
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
