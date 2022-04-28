
/* qqq : xxx : move to wtools::mem. discuss */

///
/// Are two pointers points on the same region.
///

pub fn mem_same_ptrs< T1, T2 >( src1 : &T1, src2 : &T2 ) -> bool
{
  unsafe
  {
    std::mem::transmute::< *const T1, *const () >( src1 as *const T1 )
    ==
    std::mem::transmute::< *const T2, *const () >( src2 as *const T2 )
    // std::mem::transmute::< *const T1, *const T2 >( src1 as *const T1 ) == src2 as *const T2
  }
}

///
/// Are two pointers points on the same region.
///

pub fn mem_same_size< T1, T2 >( _src1 : &T1, _src2 : &T2 ) -> bool
{
  core::mem::size_of::< T1 >() == core::mem::size_of::< T2 >()
}

///
/// Are two pointers points on the same region.
///

pub fn mem_same< T1, T2 >( src1 : &T1, src2 : &T2 ) -> bool
{
  mem_same_ptrs( src1, src2 ) && mem_same_size( src1, src2 )
}