use crate::utils::error;
use itertools::Itertools;
use syn::Result;

pub struct Model {
    items: Vec<Item>,
}

impl Model {
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        return self.items.iter();
    }

    pub fn from_syn(input: &[syn::Item]) -> Result<Self> {
        let items = input
            .iter()
            .filter_map(|item| match &item {
                syn::Item::Struct(struct_item) => Some(Item::from_syn_struct(struct_item)),
                syn::Item::Enum(enum_item) => Some(Item::from_syn_enum(enum_item)),
                _ => None,
            })
            .try_collect()?;

        return Ok(Model { items });
    }
}

pub enum Item {
    Struct {
        name: syn::Ident,
        attributes: Vec<syn::Attribute>,
        fields: Vec<Field>,
    },
    Enum {
        name: syn::Ident,
        attributes: Vec<syn::Attribute>,
        variants: Vec<Variant>,
    },
}

impl Item {
    fn from_syn_struct(input: &syn::ItemStruct) -> Result<Self> {
        let syn::Fields::Named(fields) = &input.fields else {
            return error(&input.fields, "Only named fields are supported.");
        };

        return Ok(Item::Struct {
            name: input.ident.clone(),
            attributes: input.attrs.clone(),
            fields: Field::from_syn(fields),
        });
    }

    fn from_syn_enum(input: &syn::ItemEnum) -> Result<Self> {
        return Ok(Item::Enum {
            name: input.ident.clone(),
            attributes: input.attrs.clone(),
            variants: input.variants.iter().map(Variant::from_syn).try_collect()?,
        });
    }
}

pub struct Variant {
    pub name: syn::Ident,
    pub fields: Option<Vec<Field>>,
}

impl Variant {
    fn from_syn(input: &syn::Variant) -> Result<Self> {
        let name = input.ident.clone();

        match &input.fields {
            syn::Fields::Named(fields) => {
                return Ok(Variant {
                    name,
                    fields: Some(Field::from_syn(fields)),
                });
            }
            syn::Fields::Unit => {
                return Ok(Variant { name, fields: None });
            }
            syn::Fields::Unnamed(fields) => {
                return error(fields, "Unnamed fields are not supported.");
            }
        };
    }
}

pub struct Field {
    pub name: syn::Ident,
    pub tipe: syn::Type,
}

impl Field {
    fn from_syn(input: &syn::FieldsNamed) -> Vec<Self> {
        return input
            .named
            .iter()
            .map(|field| Self {
                name: field.ident.clone().unwrap(),
                tipe: field.ty.clone(),
            })
            .collect();
    }
}
