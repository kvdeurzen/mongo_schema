use quote::quote;

use crate::SchemaDefinition;

pub fn new_struct(def: &SchemaDefinition) -> proc_macro2::TokenStream {
    let struct_vis = &def.vis;
    let struct_name  = &def.ident;
    
    // Reconstruct fields without attributes
    let fields = &def.fields.clone().into_iter().fold( proc_macro2::TokenStream::new(), |acc, field| {
        let vis = &field.vis;
        let name = &field.ident;
        let ty = &field.ty;

        quote! {
            #acc
            #vis #name: #ty ,
        }
    });

    // Add id and collection field
    let id_ident = proc_macro2::Ident::new("id", proc_macro2::Span::call_site());
    let collection_ident = proc_macro2::Ident::new("collection", proc_macro2::Span::call_site());
    let fields = quote! {
        #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
        pub #id_ident: Option<ObjectId>,
        #[serde(skip_serializing, deserialize_with = "collection_deserializer::<_, Schema1>")]
        #collection_ident: Collection<Self>,
        #fields
    };

    // Return struct as token stream
    quote! {
        #[derive(Serialize, Deserialize, Clone)]
        #struct_vis struct #struct_name {
            #fields
        }
    }
}