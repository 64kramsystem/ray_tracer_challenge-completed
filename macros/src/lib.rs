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

            // The parent/children methods encapsulate (as possible) the typical access pattern.

            fn parent(&self) -> Option<Arc<dyn Shape>> {
                Weak::upgrade(&*self.parent.lock().unwrap())
            }

            fn parent_mut(&self) -> MutexGuard<Weak<dyn Shape>> {
                self.parent.lock().unwrap()
            }

            fn children(&self) -> MutexGuard<Vec<Arc<dyn Shape>>> {
                self.children.lock().unwrap()
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
