impl<T:?Sized> Inner<T> for WrappedInner<T>{
	fn inner(&self)->&T{
		let Self(t)=self;
		t
	}
}
impl<T:?Sized> InnerMut<T> for WrappedInner<T>{
	fn inner_mut(&mut self)->&mut T{
		let Self(t)=self;
		t
	}
}
impl<T> UnwrapInner<T> for WrappedInner<T>{
	fn unwrap_inner(self)->T{
		let Self(t)=self;
		t
	}
}
#[macro_export]
/// creates a transparent wrapper and implements some important traits for it// TODO allow multiple types and where bounds, make the multi call thing work
/// generic_transparent_wrapper!(@declare "comment" pub(somewhere) Wrapper<T>) declares the struct and derives builtin rust traits #[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)], applies #[repr(transparent)], and documents with the comment
/// generic_transparent_wrapper!(unsafe @from-ref Wrapper<T>) implements from &T for &Wrapper<T> and from &mut T for &mut Wrapper<T>. for safety, Wrapper<T> must be a transparent repr of T
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
	(@get $($comment:literal)? $v:vis $name:ident<$param:ident>$(where $($bound:tt)*)?)=>(
		impl<$param:Clone> GetInner<$param> for $name<$param>$(where $($bound)*)?{
			fn get_inner(&self)->$param{self.0.clone()}
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
	($(@$variant:ident)* $name:ident<$param:ident>)=>($(
		generic_transparent_wrapper!(@$variant $name<$param>);
	)*);
}
generic_transparent_wrapper!(@declare "wrapper to make unwrap_inner and other access functions that might just call versions of themselves on the inner value stop here" pub WrappedInner<T> where T:?Sized);
generic_transparent_wrapper!(@from @get WrappedInner<T>);
generic_transparent_wrapper!(unsafe @from-ref WrappedInner<T> where T:?Sized);
pub use generic_transparent_wrapper;
use core::mem;
use crate::access::*;
