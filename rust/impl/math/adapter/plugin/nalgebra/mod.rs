
//!
//! Implement adapters for `nalgebra`.
//!

/// Internal namespace.
pub( crate ) mod private
{

  /// X2 Vector of nalgebra
  pub type X2< Scalar > = ::nalgebra::Vector2< Scalar >;

}

//

crate::mod_interface!
{

  layer as_foreign;
  #[ cfg( nalgebra_ops ) ]
  layer ops;
  layer x2;

  protected use X2;
  protected use ::nalgebra::*;

}
