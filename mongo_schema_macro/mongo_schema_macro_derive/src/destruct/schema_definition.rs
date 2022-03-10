use syn::{braced, token, Field, Ident, Result, Token, Visibility};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

#[derive(Debug)]
pub struct SchemaDefinition {
    pub vis: Visibility,
    pub struct_token: Token![struct],
    pub ident: Ident,
    pub brace_token: token::Brace,
    pub fields: Punctuated<Field, Token![,]>,
}


impl Parse for SchemaDefinition {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(SchemaDefinition {
            vis: input.parse()?,
            struct_token: input.parse()?,
            ident: input.parse()?,
            brace_token: braced!(content in input),
            fields: content.parse_terminated(Field::parse_named)?,
        })
    }
}