use crate::{DeserializeOver, DeserializeOverWrapper};
use serde::de::{SeqAccess, Visitor};
use serde::Deserializer;
use std::fmt;

macro_rules! tuple_impl {
  ( $( $x:ident ),+ ) => {
    #[allow(non_snake_case)]
    impl<'de, $( $x, )+> DeserializeOver<'de> for ($( $x, )+)
    where
      $( $x: DeserializeOver<'de>, )+
    {
      fn deserialize_over<De>(&mut self, de: De) -> Result<(), De::Error>
      where
        De: Deserializer<'de>,
      {
        const LEN: usize = 0 $( + if false { stringify!($x).len() } else { 1 } )+;

        struct TupleVisitor<'a, T>(&'a mut T);

        impl<'a, 'de, $( $x, )+> Visitor<'de> for TupleVisitor<'a, ($( $x, )+)>
        where
          $( $x: DeserializeOver<'de>, )+
        {
          type Value = ();

          fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
            fmt.write_fmt(format_args!("a tuple of length {}", LEN))
          }

          fn visit_seq<Ac>(self, mut seq: Ac) -> Result<Self::Value, Ac::Error>
          where
            Ac: SeqAccess<'de>
          {
            let ($( $x, )+) = self.0;

            $(
              if seq.next_element_seed(DeserializeOverWrapper($x))?.is_none() {
                return Ok(());
              }
            )+

            Ok(())
          }
        }

        de.deserialize_tuple(LEN, TupleVisitor(self))
      }
    }
  }
}

macro_rules! tuple_impl_stacked {
  ( $x:ident ) => {
    tuple_impl!($x);
  };
  ( $head:ident, $( $rest:ident ),+ ) => {
    tuple_impl!($head, $( $rest ),+);
    tuple_impl_stacked!($( $rest ),+);
  };
}

tuple_impl_stacked!(
  X0, X1, X2, X3, X4, X5, X6, X7, X8, X9, X10, X11, X12, X13, X14, X15, X16, X17, X18, X19, X20,
  X21, X22, X23, X24, X25, X26, X27, X28, X29, X30, X31
);
