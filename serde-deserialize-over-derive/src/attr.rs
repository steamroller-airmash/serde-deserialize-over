use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Ident, LitStr, Token};

pub(crate) struct ValueOption<T> {
  pub ident: Ident,
  #[allow(dead_code)]
  pub eq: Token![=],
  pub value: T,
}

impl<T: Parse> Parse for ValueOption<T> {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    Ok(Self {
      ident: input.parse()?,
      eq: input.parse()?,
      value: input.parse()?,
    })
  }
}

impl<T: ToTokens> ToTokens for ValueOption<T> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.ident.to_tokens(tokens);
    self.eq.to_tokens(tokens);
    self.value.to_tokens(tokens);
  }
}

pub(crate) enum SerdeOption {
  Flag(Ident),
  String(ValueOption<LitStr>),
}

impl Parse for SerdeOption {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek2(Token![=]) {
      input.parse().map(SerdeOption::String)
    } else {
      input.parse().map(SerdeOption::Flag)
    }
  }
}

impl ToTokens for SerdeOption {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      Self::Flag(tag) => tag.to_tokens(tokens),
      Self::String(opt) => opt.to_tokens(tokens),
    }
  }
}

impl SerdeOption {
  pub fn ident(&self) -> &Ident {
    match self {
      Self::Flag(tag) => tag,
      Self::String(opt) => &opt.ident,
    }
  }

  pub fn is_flag(&self) -> bool {
    match self {
      Self::Flag(_) => true,
      _ => false,
    }
  }

  #[allow(dead_code)]
  pub fn is_opt(&self) -> bool {
    match self {
      Self::String(_) => true,
      _ => false,
    }
  }
}

pub(crate) struct SerdeAttrBody {
  #[allow(dead_code)]
  pub paren: syn::token::Paren,
  pub attrs: Punctuated<SerdeOption, Token![,]>,
}

impl SerdeAttrBody {
  #[allow(dead_code)]
  pub fn has(&self, name: &str) -> bool {
    for attr in &self.attrs {
      if let SerdeOption::Flag(tag) = attr {
        if tag.to_string() == name {
          return true;
        }
      }
    }

    false
  }

  pub fn get(&self, name: &str) -> Option<&LitStr> {
    for attr in &self.attrs {
      if let SerdeOption::String(opt) = attr {
        if opt.ident.to_string() == name {
          return Some(&opt.value);
        }
      }
    }

    None
  }

  #[allow(dead_code)]
  pub fn span_for(&self, name: &str) -> Span {
    self
      .attrs
      .iter()
      .find(|attr| attr.ident().to_string() == name)
      .map(|attr| attr.span())
      .unwrap_or_else(Span::call_site)
  }
}

impl Parse for SerdeAttrBody {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let inner;

    Ok(Self {
      paren: syn::parenthesized!(inner in input),
      attrs: Punctuated::parse_terminated(&inner)?,
    })
  }
}
