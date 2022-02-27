//! Derive macros for serde-deserialize-over.

#![recursion_limit = "256"]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::quote;
use syn::{
  parse_macro_input, parse_quote, Attribute, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed,
  Ident, Meta, Type,
};

const CRATE_NAME: &str = "serde_deserialize_over";

/// Derive macro for the `DeserializeOver` trait.
#[proc_macro_derive(DeserializeOver, attributes(deserialize_over))]
pub fn derive(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let crate_name =
    crate_name("serde-deserialize-over").unwrap_or(FoundCrate::Name(CRATE_NAME.to_string()));
  let crate_name = match crate_name {
    FoundCrate::Name(name) => Ident::new(&name, Span::call_site()),
    FoundCrate::Itself => Ident::new(CRATE_NAME, Span::call_site()),
  };

  let data = match input.data {
    Data::Struct(ref data) => data.clone(),
    Data::Enum(_) => panic!("`DeserializeOver` cannot be automatically derived for enums"),
    Data::Union(_) => panic!("`DeserializeOver` cannot be automatically derived for unions"),
  };

  let res = match data.fields {
    Fields::Named(fields) => impl_named_fields(input, crate_name, fields),
    Fields::Unnamed(fields) => impl_unnamed_fields(input, crate_name, fields),
    Fields::Unit => impl_unit(input, crate_name),
  };

  match res {
    Ok(res) => {
      // panic!("{}", res);
      res
    }
    Err(e) => e.to_compile_error().into(),
  }
}

#[derive(Clone)]
struct FieldInfo {
  name: Ident,
  ty: Type,
  passthrough: bool,
}

fn impl_generic(
  mut input: DeriveInput,
  real_crate_name: Ident,
  fields: Vec<FieldInfo>,
  fields_numbered: bool,
) -> syn::Result<TokenStream> {
  let struct_name = &input.ident;
  let deserializer = Ident::new("__deserializer", Span::call_site());
  let crate_name = Ident::new(&("_".to_owned() + CRATE_NAME), Span::call_site());
  let export = quote! { #crate_name::export };

  let field_enums = fields
    .iter()
    .cloned()
    .enumerate()
    .map(|(idx, x)| Ident::new(&format!("__field{}", idx), x.name.span()))
    .collect::<Vec<_>>();
  let field_enums = &field_enums;
  let field_enums_copy1 = field_enums;
  let field_enums_copy2 = field_enums;
  let field_names = fields.iter().map(|x| &x.name).cloned().collect::<Vec<_>>();
  let field_names = &field_names;
  let indices = (0usize..fields.len()).collect::<Vec<_>>();
  let indices_u64 = indices.iter().map(|x| *x as u64);

  let missing_field_error_str = syn::LitStr::new(
    &format!("field index between 0 <= i < {}", fields.len()),
    Span::call_site(),
  );

  let visit_str_and_bytes_impl = if !fields_numbered {
    let names_str = field_names
      .iter()
      .map(|x| syn::LitStr::new(&x.to_string(), x.span()))
      .collect::<Vec<_>>();
    let names_bytes = field_names
      .iter()
      .map(|x| syn::LitByteStr::new(x.to_string().as_bytes(), x.span()))
      .collect::<Vec<_>>();

    quote! {
      fn visit_str<E>(self, value: &str) -> #export::Result<Self::Value, E>
      where
        E: #export::Error
      {
        #export::Ok(match value {
          #( #names_str => __Field::#field_enums, )*
          _ => __Field::__ignore
        })
      }

      fn visit_bytes<E>(self, value: &[u8]) -> #export::Result<Self::Value, E>
      where
        E: #export::Error
      {
        #export::Ok(match value {
          #( #names_bytes => __Field::#field_enums, )*
          _ => __Field::__ignore
        })
      }
    }
  } else {
    quote! {}
  };

  let map_de_entries = fields
    .iter()
    .map(|field| {
      let ref name = field.name;
      if field.passthrough {
        quote! {
          map.next_value_seed(
            #crate_name::DeserializeOverWrapper(&mut (self.0).#name)
          )?
        }
      } else {
        quote! { (self.0).#name = map.next_value()? }
      }
    })
    .collect::<Vec<_>>();

  let visit_seq_entries = fields
    .iter()
    .map(|field| {
      let ref name = field.name;
      if field.passthrough {
        quote! {
          match seq.next_element_seed(
            #crate_name::DeserializeOverWrapper(&mut (self.0).#name)
          )? {
            Some(()) => (),
            None => return Ok(())
          }
        }
      } else {
        quote! {
          (self.0).#name = match seq.next_element()? {
            Some(x) => x,
            None => return Ok(())
          }
        }
      }
    })
    .collect::<Vec<_>>();

  if !input.generics.params.is_empty() {
    let where_clause = input.generics.make_where_clause();

    for field in fields.iter() {
      let ty = &field.ty;

      if field.passthrough {
        where_clause.predicates.push(parse_quote! {
          #ty: #crate_name::DeserializeOver<'de>
        });
      } else {
        where_clause.predicates.push(parse_quote! {
          #ty: #crate_name::export::Deserialize<'de>
        });
      }
    }
  }

  let (_, ty_generics, where_clause) = input.generics.split_for_impl();
  let impl_generics = &input.generics.params;

  let inner = quote! {
    #[allow(unknown_lints)]
    #[allow(rust_2018_idioms)]
    extern crate #real_crate_name as #crate_name;

    #[automatically_derived]
    impl<'de, #impl_generics> #crate_name::DeserializeOver<'de> for #struct_name #ty_generics
      #where_clause
    {
      fn deserialize_over<D>(&mut self, #deserializer: D) -> #export::Result<(), D::Error>
      where
        D: #export::Deserializer<'de>
      {
        #[allow(non_camel_case_types)]
        enum __Field {
          #( #field_enums, )*
          __ignore
        }
        impl<'de> #export::Deserialize<'de> for __Field {
          fn deserialize<D>(#deserializer: D) -> #export::Result<Self, D::Error>
          where
            D: #export::Deserializer<'de>
          {
            #export::Deserializer::deserialize_identifier(#deserializer, __FieldVisitor)
          }
        }

        struct __FieldVisitor;
        impl<'de> #export::Visitor<'de> for __FieldVisitor {
          type Value = __Field;

          fn expecting(&self, fmt: &mut #export::fmt::Formatter) -> #export::fmt::Result {
            #export::fmt::Formatter::write_str(fmt, "field identifier")
          }

          fn visit_u64<E>(self, value: u64) -> #export::Result<Self::Value, E>
          where
            E: #export::Error
          {
            use #export::{Ok, Err};

            Ok(match value {
              #( #indices_u64 => __Field::#field_enums, )*
              _ => return Err(#export::Error::invalid_value(
                #export::Unexpected::Unsigned(value),
                &#missing_field_error_str
              ))
            })
          }

          #visit_str_and_bytes_impl
        }

        struct __Visitor<'a, #impl_generics>(pub &'a mut #struct_name #ty_generics);

        impl<'a, 'de, #impl_generics> #export::Visitor<'de> for __Visitor<'a, #impl_generics>
          #where_clause
        {
          type Value = ();

          fn expecting(&self, fmt: &mut #export::fmt::Formatter) -> #export::fmt::Result {
            #export::fmt::Formatter::write_str(fmt, concat!("struct ", stringify!(#struct_name)))
          }

          fn visit_seq<A>(self, mut seq: A) -> #export::Result<Self::Value, A::Error>
          where
            A: #export::SeqAccess<'de>
          {
            use #export::{Some, None};

            #( #visit_seq_entries; )*

            Ok(())
          }

          fn visit_map<A>(self, mut map: A) -> #export::Result<Self::Value, A::Error>
          where
            A: #export::MapAccess<'de>
          {
            use #export::{Some, None, Error};

            // State tracking
            #(
              let mut #field_enums: bool = false;
            )*

            while let Some(key) = map.next_key::<__Field>()? {
              match key {
                #(
                  __Field::#field_enums => if #field_enums_copy1 {
                    return Err(<A::Error as Error>::duplicate_field(stringify!(#field_names)));
                  } else {
                    #field_enums_copy2 = true;
                    #map_de_entries;
                  }
                )*
                _ => (),
              }
            }

            Ok(())
          }
        }

        const FIELDS: &[&str] = &[
          #( stringify!(#field_names), )*
        ];

        #export::Deserializer::deserialize_struct(
          #deserializer,
          stringify!(#struct_name),
          FIELDS,
          __Visitor(self)
        )
      }
    }
  };

  let const_name = Ident::new(
    &format!("_IMPL_DESERIALIZE_OVER_FOR_{}", struct_name),
    struct_name.span(),
  );

  Ok(
    quote! {
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
      const #const_name: () = {
        #inner
      };
    }
    .into(),
  )
}

fn impl_named_fields(
  input: DeriveInput,
  crate_name: Ident,
  fields: FieldsNamed,
) -> syn::Result<TokenStream> {
  let fieldinfos = fields
    .named
    .iter()
    .map(|x| {
      let attrinfo = parse_attr(x.attrs.iter())?;

      Ok(FieldInfo {
        name: x.ident.clone().unwrap(),
        ty: x.ty.clone(),
        passthrough: attrinfo.use_deserialize_over,
      })
    })
    .collect::<Result<Vec<_>, syn::Error>>()?;

  return impl_generic(input, crate_name, fieldinfos, false);
}

fn impl_unnamed_fields(
  _input: DeriveInput,
  _crate_name: Ident,
  _fields: FieldsUnnamed,
) -> syn::Result<TokenStream> {
  unimplemented!()
}

fn impl_unit(input: DeriveInput, crate_name: Ident) -> syn::Result<TokenStream> {
  let struct_name = &input.ident;

  Ok(
    quote! {
      impl ::#crate_name::DeserializeOver for #struct_name {
        fn deserialize_over<'de, D>(&mut self, de: D) -> Result<(), D::Error>
        where
          D: Deserializer<'de>
        {
          Ok(())
        }
      }
    }
    .into(),
  )
}

#[derive(Default)]
struct ParsedAttr {
  use_deserialize_over: bool,
}

fn parse_attr<'a, I>(attrs: I) -> syn::Result<ParsedAttr>
where
  I: Iterator<Item = &'a Attribute>,
{
  let mut result = ParsedAttr::default();

  for attr in attrs.into_iter() {
    match attr.parse_meta()? {
      Meta::Path(ref path) => {
        if path.is_ident("deserialize_over") {
          result.use_deserialize_over = true;
        }
      }
      Meta::List(_) | Meta::NameValue(_) => (),
    }
  }

  Ok(result)
}
