extern crate proc_macro;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data, parse_macro_input};
use syn::{Attribute, Lit, Meta, MetaNameValue};
use regex::Regex;

#[proc_macro_derive(CustomEq, attributes(ignore_regex))]
pub fn custom_struct_eq(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Collect regex pattern from attribute or set to default value
    let re_attr: Option<Attribute> = input.attrs.iter()
        .find(|attr| {
            attr.path.is_ident("ignore_regex")
        })
        .map(|attr| (*attr).clone());

    let regex_str = if let Some(attribute) = re_attr {
        // Parse the attribute into a Meta object
        let meta = attribute.parse_meta().unwrap();

        // Extract the attribute value from the Meta object
        if let Meta::NameValue(
            MetaNameValue { lit: Lit::Str(lit_str), .. }
        ) = meta {
            lit_str.value()
        } else {
            panic!("Attribute value is not a string literal");
        }
    } else {
        // Use a catch-none regex if the attribute is not present
        String::from("^$")
    };

    let regex = Regex::new(&regex_str).expect("Invalid Regex");
    let name = &input.ident;
    let mut fields = vec![];

    // collect all struct field names that don't match the regex
    match input.data {
        Data::Struct(ref data) => {
            for field in data.fields.iter() {
                if let Some(ref ident) = field.ident {
                    let field_name = ident.to_string();
                    if !regex.is_match(&field_name) {
                        fields.push(ident);
                    }
                }
            }
        }
        _ => {}
    };

    let output = quote! {
        impl Eq for #name {}
        impl PartialEq for #name {
            fn eq(&self, other: &Self) -> bool {
                #(self.#fields == other.#fields)&&*
            }
        }
    };

    let stream: TokenStream = output.into();
    return stream;
}