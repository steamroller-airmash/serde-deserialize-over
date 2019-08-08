
#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::crate_name;
use syn::{
  parse_macro_input, DeriveInput, Data, Fields, Ident,
  FieldsNamed, FieldsUnnamed, Attribute, Meta, Error
};
use quote::quote;

#[proc_macro_derive(DeserializeOver, attributes(deserialize_over))]
pub fn derive(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let crate_name = crate_name("serde-deserialize-over")
    .unwrap_or("serde_deserialize_over".to_string());
  let crate_name = Ident::new(&crate_name, Span::call_site());

  let struct_name = input.ident;

  let data = match input.data {
    Data::Struct(data) => data,
    Data::Enum(_) => panic!("`DeserializeOver` cannot be automatically derived for enums"),
    Data::Union(_) => panic!("`DeserializeOver` cannot be automatically derived for unions")
  };

  let res = match data.fields {
    Fields::Named(fields) => {
      impl_named_fields(struct_name, crate_name, fields)
    },
    Fields::Unnamed(fields) => {
      impl_unnamed_fields(struct_name, crate_name, fields)
    },
    Fields::Unit => impl_unit(struct_name, crate_name)
  };

  match res {
    Ok(res) => res,
    Err(e) => e.to_compile_error().into()
  }
}

fn impl_named_fields(
  struct_name: Ident, 
  crate_name: Ident, 
  fields: FieldsNamed
) -> syn::Result<TokenStream> {
  let map = Ident::new("map", Span::call_site());
  let visitor = visitor_name();

  let fields = fields.named
    .iter()
    .map(|field| {
      let attrs = parse_attr(field.attrs.iter())?;
      let field_name = field.ident.as_ref().unwrap();

      Ok(if attrs.use_deserialize_over {
        quote! {
          stringify!(#field_name) => #map.next_value_seed(
            ::#crate_name::DeserializeOverWrapper(&mut (self.0).#field_name)
          )?
        }
      } else {
        quote! {
          stringify!(#field_name) => (self.0).#field_name = #map.next_value()?
        }
      })
    })
    .collect::<Result<Vec<proc_macro2::TokenStream>, Error>>()?;

  Ok(quote! {
    impl ::#crate_name::DeserializeOver for #struct_name {
      fn deserialize_over<'de, D>(&mut self, de: D) -> Result<(), D::Error>
      where
        D: ::#crate_name::serde::Deserializer<'de>
      {
        struct #visitor<'a>(pub &'a mut #struct_name);

        impl<'a, 'de> ::#crate_name::serde::de::Visitor<'de> for #visitor<'a> {
          type Value = ();

          fn expecting(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(fmt, concat!("an instance of ", stringify!(#struct_name)))
          }

          fn visit_map<A>(self, mut #map: A) -> Result<(), A::Error>
          where
            A: ::#crate_name::serde::de::MapAccess<'de>
          {
            while let Some(key) = #map.next_key::<&str>()? {
              match key {
                #( #fields, )*
                _ => ()
              }
            }

            Ok(())
          }
        }
      
        de.deserialize_map(#visitor(self))
      }
    }
  }.into())
}

fn impl_unnamed_fields(
  _struct_name: Ident,
  _crate_name: Ident,
  _fields: FieldsUnnamed
) -> syn::Result<TokenStream> {
  unimplemented!()
}

fn impl_unit(
  struct_name: Ident,
  crate_name: Ident
) -> syn::Result<TokenStream> {
  Ok(quote! {
    impl ::#crate_name::DeserializeOver for #struct_name {
      fn deserialize_over<'de, D>(&mut self, de: D) -> Result<(), D::Error>
      where
        D: Deserializer<'de>
      {
        Ok(())
      }
    }
  }.into())
}

#[derive(Default)]
struct ParsedAttr {
  use_deserialize_over: bool
}

fn parse_attr<'a, I>(attrs: I) -> syn::Result<ParsedAttr> 
where
  I: Iterator<Item = &'a Attribute>
{
  let mut result = ParsedAttr::default();

  for attr in attrs.into_iter() {
    let meta = attr.parse_meta()?;
    let name = meta.name();

    match meta {
      Meta::Word(ref ident) if ident == "deserialize_over" => {
        result.use_deserialize_over = true;
      }
      Meta::Word(_) => (),
      Meta::List(_) | Meta::NameValue(_) => {
        return Err(Error::new(name.span(), "invalid deserialize_over attribute"));
      }
    }
  }

  Ok(result)
}

fn visitor_name() -> Ident {
  Ident::new("_Serde_Deserialize_Over_Visitor_109210701893", Span::call_site())
}
