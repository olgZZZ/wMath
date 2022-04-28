/// Internal namespace.
pub mod internal
{
  use core::mem::size_of;
  use crate::prelude::*;
  use crate::x2;
  use crate::ScalarInterface;

  include!( "./cgmath_x2.rs" );
  impl_x2_for!( cgmath::Vector2< Scalar > );

}

/// Exposed namespace of the module.
pub mod exposed
{
  // use super::internal as i;
}

pub use exposed::*;

/// Prelude to use: `use wtools::prelude::*`.
pub mod prelude
{
  // use super::internal as i;
}