use crate::{DeserializeOver, DeserializeOverWrapper};
use serde::de::{SeqAccess, Visitor};
use serde::Deserializer;
use std::fmt;

impl<'de> DeserializeOver<'de> for () {
  fn deserialize_over<D>(&mut self, de: D) -> Result<(), D::Error>
  where
    D: Deserializer<'de>,
  {
    struct NoopVisitor;

    impl<'de> Visitor<'de> for NoopVisitor {
      type Value = ();

      fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("a unit")
      }

      fn visit_unit<E>(self) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        Ok(())
      }
    }

    de.deserialize_unit(NoopVisitor)
  }
}

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

tuple_impl!(A);
tuple_impl!(A, B);
tuple_impl!(A, B, C);
tuple_impl!(A, B, C, D);
tuple_impl!(A, B, C, D, E);
tuple_impl!(A, B, C, D, E, F);
tuple_impl!(A, B, C, D, E, F, G);
tuple_impl!(A, B, C, D, E, F, G, H);
tuple_impl!(A, B, C, D, E, F, G, H, I);
tuple_impl!(A, B, C, D, E, F, G, H, I, J);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
tuple_impl!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
