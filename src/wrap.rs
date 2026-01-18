direct_ref!(@inner WrappedInner<T> where T:?Sized);
direct_ref!(@mut WrappedInner<T> where T:?Sized);

#[macro_export]
/// creates a transparent wrapper and implements some important traits for it TODO allow multiple types and where bounds, make the multi call thing work
/// generic_transparent_wrapper!(@declare "comment" pub(somewhere) Wrapper<T>) declares the struct and derives builtin rust traits #[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)], applies #[repr(transparent)], and documents with the comment
/// generic_transparent_wrapper!(unsafe @from-ref Wrapper<T>) implements from &T for &Wrapper<T> and from &mut T for &mut Wrapper<T>. for safety, Wrapper<T> must be a transparent repr of T
/// TODO document other branches
/// example usage: creating WrappedInner

/// generic_transparent_wrapper!(@declare "wrapper to make unwrap_inner and other access functions that might just call versions of themselves on the inner value stop here" pub WrappedInner<T> where T:?Sized);
/// generic_transparent_wrapper!(@from WrappedInner<T>);
/// generic_transparent_wrapper!(@get WrappedInner<T>.0);
/// generic_transparent_wrapper!(@inner WrappedInner<T>.0 where T:?Sized);
/// generic_transparent_wrapper!(@mut WrappedInner<T>.0 where T:?Sized);
/// generic_transparent_wrapper!(@unwrap WrappedInner<T>.0 where T:?Sized);
/// generic_transparent_wrapper!(unsafe @from-ref WrappedInner<T> where T:?Sized);

macro_rules! generic_transparent_wrapper{
	(@declare $($comment:literal)? $v:vis $name:ident<$param:ident>$(where $($bound:tt)*)?)=>(
		#[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)]
		$(#[doc=$comment])?
		#[repr(transparent)]
		$v struct $name<$param>($v $param)$(where $($bound)*)?;
	);
	(@declare-no-derive $($comment:literal)? $v:vis $name:ident<$param:ident>$(where $($bound:tt)*)?)=>(
		$(#[doc=$comment])?
		#[repr(transparent)]
		$v struct $name<$param>($v $param)$(where $($bound)*)?;
	);
	(@declare-serial $($comment:literal)? $v:vis $name:ident<$param:ident>$(where $($bound:tt)*)?)=>(
		#[derive(Clone,Copy,Debug,Default,Deserialize,Eq,Hash,Ord,PartialEq,PartialOrd,Serialize)]
		$(#[doc=$comment])?
		#[repr(transparent)]
		$v struct $name<$param:?Sized>($v $param);
	);
	(@from $($comment:literal)? $v:vis $name:ident<$param:ident>$(where $($bound:tt)*)?)=>(
		impl<$param:Sized> From<$param> for $name<$param>$(where $($bound)*)?{
			fn from(value:$param)->Self{Self(value)}
		}
	);
	(@get $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param:Clone> GetInner<$param> for $name<$param>$(where $($bound)*)?{
			fn get_inner(&self)->$param{self.$field.clone()}
		}
	);
	(@inner $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param:Clone> Inner<$param> for $name<$param>$(where $($bound)*)?{
			fn inner(&self)->&$param{&self.$field}
		}
	);
	(@mut $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param:Clone> InnerMut<$param> for $name<$param>$(where $($bound)*)?{
			fn inner_mut(&mut self)->&mut $param{&mut self.$field}
		}
	);
	(unsafe @from-ref $name:ident<$param:ident>$(where $($bound:tt)*)?)=>(
		impl<$param> From<&$param> for &$name<$param>$(where $($bound)*)?{
			fn from(value:&$param)->Self{
				unsafe{mem::transmute(value)}		// transparent repr cast
			}
		}
		impl<$param> From<&mut T> for &mut $name<$param>$(where $($bound)*)?{
			fn from(value:&mut $param)->Self{
				unsafe{mem::transmute(value)}		// transparent repr cast
			}
		}
	);
	(@unwrap $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param:Sized> UnwrapInner<$param> for $name<$param>$(where $($bound)*)?{
			fn unwrap_inner(self)->$param{self.$field}
		}
	);
}
generic_transparent_wrapper!(@declare "wrapper to make unwrap_inner and other access functions that might just call versions of themselves on the inner value stop here" pub WrappedInner<T> where T:?Sized);
generic_transparent_wrapper!(@from WrappedInner<T> where T:?Sized);
generic_transparent_wrapper!(@get WrappedInner<T>.0);
generic_transparent_wrapper!(@inner WrappedInner<T>.0 where T:?Sized);
generic_transparent_wrapper!(@mut WrappedInner<T>.0 where T:?Sized);
generic_transparent_wrapper!(@unwrap WrappedInner<T>.0);
generic_transparent_wrapper!(unsafe @from-ref WrappedInner<T> where T:?Sized);
pub use generic_transparent_wrapper;
self_inner!(@get WrappedInner<T>);
self_inner!(@inner WrappedInner<T> where T:?Sized);
self_inner!(@mut WrappedInner<T> where T:?Sized);
use core::mem;
use crate::access::*;
