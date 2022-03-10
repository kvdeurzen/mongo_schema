extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::{
    AttributeArgs,
    parse_macro_input
};

mod destruct;
use destruct::{
    parse_attribute,
    SchemaAttribute,
    SchemaDefinition,
};

mod construct;


#[proc_macro_attribute]
pub fn mongo_schema(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the struct attributes
    let attr = parse_macro_input!(attr as AttributeArgs);
    let schema_attributes = parse_attribute(&attr);
    
    // Parse the struct itself
    let schema_definition = parse_macro_input!(item as SchemaDefinition);

    impl_mongo_schema_macro(schema_definition, schema_attributes)
}

fn impl_mongo_schema_macro(def: SchemaDefinition, attr: SchemaAttribute) -> TokenStream {
    let imports = construct::imports();
    let struct_new = construct::new_struct(&def);
    let struct_impl = construct::struct_impl(&def, &attr);
    let mongo_schema_impl = construct::mongo_schema_impl(&def, &attr);

    quote! {
        #imports
        #struct_new
        #struct_impl
        #mongo_schema_impl
    }.into()
}

#[allow(dead_code)]
fn string_to_token(input: String) -> proc_macro2::TokenStream {
    input.parse().unwrap()
}

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
