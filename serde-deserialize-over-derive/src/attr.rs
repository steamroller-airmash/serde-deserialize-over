use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
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
