use darling::FromMeta;
use syn::NestedMeta;

#[derive(Debug, Default, FromMeta)]
#[darling(default)]
pub struct SchemaAttribute {
    pub collection: Option<String>,
    pub check: bool
}

pub fn parse_attribute(attributes: &[NestedMeta]) -> SchemaAttribute {
    SchemaAttribute::from_list(attributes).unwrap()
}