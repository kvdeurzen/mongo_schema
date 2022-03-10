use quote::quote;

use crate::{
    SchemaAttribute,
    SchemaDefinition,
};

pub fn struct_impl(def: &SchemaDefinition, _attr: &SchemaAttribute) -> proc_macro2::TokenStream {
    let struct_name  = &def.ident;

    // Build arguments and fields
    // The arguments are used in the construction of the `new` constructor method
    // The construct_fields are used to construct the struct in the `new` constructor method
    // The debug_fields are used to build the debug method. A custom debug allows for some fields to stay hidden (For example the mongo collection).
    let (arguments, construct_fields, debug_fields) = &def
        .fields
        .clone()
        .into_iter()
        .fold( (proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new(), proc_macro2::TokenStream::new()), |acc, field| {
            let name = &field.ident;
            let name_stringed = name.clone().unwrap().to_string();
            let ty = &field.ty;

            let arguments = acc.0;
            let construct_fields = acc.1;
            let debug_fields = acc.2;

            let arguments = quote! {
                #arguments
                #name: #ty ,
            };
            let construct_fields = quote! {
                #construct_fields
                #name ,
            };
            let debug_fields = quote! {
                #debug_fields
                .field( #name_stringed , &self.#name)
            };

            (arguments, construct_fields, debug_fields)
        });

    quote! {
        impl #struct_name {
            pub fn new( #arguments ) -> Self {
                Self{
                    id: None,
                    collection: MONGO_DATABASE.get().expect("Mongo client hasn't been initialized.").collection::<Self>(Self::collection_name()),
                    #construct_fields
                }
            }
        }

        impl std::fmt::Debug for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!(#struct_name))
                    .field("id", &self.id)
                    .field("mongo_initiated", &MONGO_DATABASE.get().is_some())
                    #debug_fields
                    .finish()
            }
        }
    }
}