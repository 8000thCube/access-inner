direct_ref!(@inner WrappedInner<T> where T:?Sized);
direct_ref!(@mut WrappedInner<T> where T:?Sized);

#[macro_export]
/// creates a transparent wrapper and implements some important traits for it TODO allow multiple types and where bounds, make the multi call thing work
/// generic_wrapper!(@declare "comment" pub(somewhere) Wrapper<T>) declares the struct and derives builtin rust traits #[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)], applies #[repr(transparent)], and documents with the comment
/// generic_wrapper!(unsafe @from-ref Wrapper<T>) implements from &T for &Wrapper<T> and from &mut T for &mut Wrapper<T>. for safety, Wrapper<T> must be a transparent repr of T
/// TODO document other branches
/// example usage: creating WrappedInner

/// direct_ref!(@inner WrappedInner<T> where T:?Sized);
/// direct_ref!(@mut WrappedInner<T> where T:?Sized);
/// generic_wrapper!(@declare "wrapper to make unwrap_inner and other access functions that might just call versions of themselves on the inner value stop here" pub WrappedInner<T> where T:?Sized);
/// generic_wrapper!(@from WrappedInner<T>);
/// generic_wrapper!(@get-stop WrappedInner<T>.0);
/// generic_wrapper!(@inner-stop WrappedInner<T>.0 where T:?Sized);
/// generic_wrapper!(@mut-stop WrappedInner<T>.0 where T:?Sized);
/// generic_wrapper!(@unwrap-stop WrappedInner<T>.0 where T:?Sized);
/// generic_wrapper!(unsafe @from-ref WrappedInner<T> where T:?Sized);
/// self_inner!(@get WrappedInner<T>);
/// self_inner!(@inner WrappedInner<T> where T:?Sized);
/// self_inner!(@mut WrappedInner<T> where T:?Sized);

macro_rules! generic_wrapper{
	(@as-mut $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param> AsMut<$param> for $name<$param>$(where $($bound)*)?{
			fn as_mut(&mut self)->&mut $param{&mut self.$field}
		}
	);
	(@as-ref $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param> AsRef<$param> for $name<$param>$(where $($bound)*)?{
			fn as_ref(&self)->&$param{&self.$field}
		}
	);
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
	(@get-stop $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param:Clone> GetInner<$param> for $name<$param>$(where $($bound)*)?{
			fn get_inner(&self)->$param{self.$field.clone()}
		}
	);
	(@get $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param:GetInner<T307EC305B4556985>,T307EC305B4556985> GetInner<T307EC305B4556985> for $name<$param>$(where $($bound)*)?{
			fn get_inner(&self)->$param{self.$field.get_inner()}
		}
	);
	(@inner-stop $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param> Inner<$param> for $name<$param>$(where $($bound)*)?{
			fn inner(&self)->&$param{&self.$field}
		}
	);
	(@inner $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param:Inner<T307EC305B4556985>,T307EC305B4556985> Inner<T307EC305B4556985> for $name<$param>$(where $($bound)*)?{
			fn inner(&self)->&T307EC305B4556985{self.$field.inner()}
		}
	);
	(@mut-stop $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param> InnerMut<$param> for $name<$param>$(where $($bound)*)?{
			fn inner_mut(&mut self)->&mut $param{&mut self.$field}
		}
	);
	(@mut $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param:InnerMut<T307EC305B4556985>,T307EC305B4556985> InnerMut<T307EC305B4556985> for $name<$param>$(where $($bound)*)?{
			fn inner_mut(&mut self)->&mut T307EC305B4556985{self.$field.inner_mut()}
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
	(@unwrap-stop $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param:Sized> UnwrapInner<$param> for $name<$param>$(where $($bound)*)?{
			fn unwrap_inner(self)->$param{self.$field}
		}
	);
	(@unwrap $($comment:literal)? $v:vis $name:ident<$param:ident>.$field:tt$(where $($bound:tt)*)?)=>(
		impl<$param:UnwrapInner<T307EC305B4556985>> UnwrapInner<T307EC305B4556985> for $name<$param>$(where $($bound)*)?{
			fn unwrap_inner(self)->$param{self.$field.unwrap_inner()}
		}
	);
}
generic_wrapper!(@declare "wrapper to make unwrap_inner and other access functions that might just call versions of themselves on the inner value stop here" pub WrappedInner<T> where T:?Sized);
generic_wrapper!(@from WrappedInner<T> where T:?Sized);
generic_wrapper!(@as-mut WrappedInner<T>.0 where T:?Sized);
generic_wrapper!(@as-ref WrappedInner<T>.0 where T:?Sized);
generic_wrapper!(@get-stop WrappedInner<T>.0);
generic_wrapper!(@inner-stop WrappedInner<T>.0 where T:?Sized);
generic_wrapper!(@mut-stop WrappedInner<T>.0 where T:?Sized);
generic_wrapper!(@unwrap-stop WrappedInner<T>.0);
generic_wrapper!(unsafe @from-ref WrappedInner<T> where T:?Sized);
pub use generic_wrapper;
self_inner!(@get WrappedInner<T>);
self_inner!(@inner WrappedInner<T> where T:?Sized);
self_inner!(@mut WrappedInner<T> where T:?Sized);
use core::mem;
use crate::access::*;
