#![ allow( unused_imports ) ]

use wtest_basic::*;
use crate::test_tools::*;
use math_adapter::prelude::*;
use math_adapter::X2;

///
/// Tests for X2 conversion function as clone_as_foreign, as_foreign, as_foreign_mut .
///

fn _convertion_foreign_test()
{
  type T = i8;
  crate::macro_x2::macro_test_x2_as_foreign!( cgmath::Vector2 ; T );
}

///
/// Operations with dereferencing.
///

fn _operation_deref_test()
{
  type T = i8;
  crate::macro_x2::macro_test_x2_operation_deref!( cgmath::Vector2 ; T );
}

//

test_suite!
{
  convertion_foreign_test,
  operation_deref_test,
}
