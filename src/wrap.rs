impl<T:Clone> GetInner<T> for WrappedInner<T>{
	fn get_inner(&self)->T{self.inner().clone()}
}
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
/// creates a transparent wrapper and implements some important traits for it// TODO allow multiple types and where bounds
/// generic_transparent_wrapper!(@declare "comment" pub(somewhere) Wrapper<T>) declares the struct and derives builtin rust traits #[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)], applies #[repr(transparent)], and documents with the comment
/// generic_transparent_wrapper!(unsafe @from-ref Wrapper<T>) implements from &T for &Wrapper<T> and from &mut T for &mut Wrapper<T>. for safety, Wrapper<T> must be a transparent repr of T
macro_rules! generic_transparent_wrapper{
	(@declare $($comment:literal)? $v:vis $name:ident<$param:ident>)=>(
		#[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)]
		$(#[doc=$comment])?
		#[repr(transparent)]
		$v struct $name<$param:?Sized>($v $param);
	);
	(@declare-no-derive $($comment:literal)? $v:vis $name:ident<$param:ident>)=>(
		$(#[doc=$comment])?
		#[repr(transparent)]
		$v struct $name<$param:?Sized>($v $param);
	);
	(@declare-serial $($comment:literal)? $v:vis $name:ident<$param:ident>)=>(
		#[derive(Clone,Copy,Debug,Default,Deserialize,Eq,Hash,Ord,PartialEq,PartialOrd,Serialize)]
		$(#[doc=$comment])?
		#[repr(transparent)]
		$v struct $name<$param:?Sized>($v $param);
	);
	(@from $name:ident<$param:ident>)=>(
		impl<$param:Sized> From<$param> for $name<$param>{
			fn from(value:$param)->Self{Self(value)}
		}
	);
	(unsafe @from-ref $name:ident<$param:ident>)=>(
		impl<$param:?Sized> From<&$param> for &$name<$param>{
			fn from(value:&$param)->Self{
				unsafe{mem::transmute(value)}		// transparent rep cast
			}
		}
		impl<$param:?Sized> From<&mut T> for &mut $name<$param>{
			fn from(value:&mut $param)->Self{
				unsafe{mem::transmute(value)}		// transparent rep cast
			}
		}
	);
}
generic_transparent_wrapper!(@declare "wrapper to make unwrap_inner and other access functions that might just call versions of themselves on the inner value stop here" pub WrappedInner<T>);
generic_transparent_wrapper!(@from WrappedInner<T>);
generic_transparent_wrapper!(unsafe @from-ref WrappedInner<T>);
pub use generic_transparent_wrapper;
use core::mem;
use crate::access::*;
