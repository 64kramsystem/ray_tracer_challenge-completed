extern crate proc_macro;

use quote::quote;
use syn;

use proc_macro::TokenStream;

#[proc_macro_derive(ShapeAccessors)]
pub fn shape_accessors_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    let gen = quote! {
        use crate::properties::Material as ShapeAccessorsMaterial;
        use crate::math::Matrix as ShapeAccessorsMatrix;
        use std::sync::Arc as ShapeAccessorsArc;
        use std::sync::MutexGuard as ShapeAccessorsMutexGuard;
        use std::sync::Weak as ShapeAccessorsWeak;

        #[cfg(test)]
        use std::any::Any as ShapeAccessorsAny;

        impl Shape for #name {
            fn id(&self) -> u32 {
                self.id
            }

            // The parent/children methods encapsulate (as possible) the typical access pattern.

            fn parent(&self) -> Option<ShapeAccessorsArc<dyn Shape>> {
                ShapeAccessorsWeak::upgrade(&*self.parent.lock().unwrap())
            }

            fn parent_mut(&self) -> ShapeAccessorsMutexGuard<ShapeAccessorsWeak<dyn Shape>> {
                self.parent.lock().unwrap()
            }

            fn children(&self) -> ShapeAccessorsMutexGuard<Vec<ShapeAccessorsArc<dyn Shape>>> {
                self.children.lock().unwrap()
            }

            fn transform(&self) -> &ShapeAccessorsMatrix {
                &self.transform
            }

            fn transform_mut(&mut self) -> &mut ShapeAccessorsMatrix {
                &mut self.transform
            }

            fn material(&self) -> &ShapeAccessorsMaterial {
                &self.material
            }

            fn material_mut(&mut self) -> &mut ShapeAccessorsMaterial {
                &mut self.material
            }

            // Not actually a Shape "accessor", but it's the exception, and this design is the simplest.
            //
            #[cfg(test)]
            fn as_any(&self) -> &dyn ShapeAccessorsAny {
                self
            }
        }
    };

    gen.into()
}
