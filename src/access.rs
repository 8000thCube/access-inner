direct_ref!(@inner Slice<T>);
direct_ref!(@inner Vec<T>);
direct_ref!(@inner bool);
direct_ref!(@inner char);
direct_ref!(@inner f32);
direct_ref!(@inner f64);
direct_ref!(@inner i128);
direct_ref!(@inner i16);
direct_ref!(@inner i32);
direct_ref!(@inner i64);
direct_ref!(@inner i8);
direct_ref!(@inner isize);
direct_ref!(@inner u128);
direct_ref!(@inner u16);
direct_ref!(@inner u32);
direct_ref!(@inner u64);
direct_ref!(@inner u8);
direct_ref!(@inner usize);
direct_ref!(@mut Slice<T>);
direct_ref!(@mut Vec<T>);
direct_ref!(@mut bool);
direct_ref!(@mut char);
direct_ref!(@mut f32);
direct_ref!(@mut f64);
direct_ref!(@mut i128);
direct_ref!(@mut i16);
direct_ref!(@mut i32);
direct_ref!(@mut i64);
direct_ref!(@mut i8);
direct_ref!(@mut isize);
direct_ref!(@mut u128);
direct_ref!(@mut u16);
direct_ref!(@mut u32);
direct_ref!(@mut u64);
direct_ref!(@mut u8);
direct_ref!(@mut usize);
impl<T:?Sized+GetInner<U>,U> GetInner<U> for Arc<T>{
	fn get_inner(&self)->U{(**self).get_inner()}
}
impl<T:?Sized+Inner<U>,U:?Sized> Inner<U> for Arc<T>{
	fn inner(&self)->&U{(**self).inner()}
}
impl<T:?Sized+InnerRefMut<U>,U:?Sized> InnerRefMut<U> for Arc<T>{
	fn inner_ref_mut(&mut self)->Self::RefMut<'_>{
		match Arc::get_mut(self){
			None=>None,
			Some(t)=>Some(t.inner_ref_mut())
		}
	}
	type RefMut<'a>=Option<T::RefMut<'a>> where T:'a,U:'a;
}
impl<T:GetInner<U>,U> GetInner<Option<U>> for Option<T>{
	fn get_inner(&self)->Option<U>{self.as_ref().map(T::get_inner)}
}
impl<T:InnerRef<U>,U:?Sized> InnerRef<U> for Option<T>{
	fn inner_ref(&self)->Self::Ref<'_>{
		match self{
			None=>None,
			Some(t)=>Some(t.inner_ref())
		}
	}
	type Ref<'a>=Option<T::Ref<'a>> where T:'a,U:'a;
}
impl<T:InnerRefMut<U>,U:?Sized> InnerRefMut<U> for Option<T>{
	fn inner_ref_mut(&mut self)->Self::RefMut<'_>{
		match self{
			None=>None,
			Some(t)=>Some(t.inner_ref_mut())
		}
	}
	type RefMut<'a>=Option<T::RefMut<'a>> where T:'a,U:'a;
}
impl<T:UnwrapInner<U>,U> UnwrapInner<Option<U>> for Arc<T>{
	fn unwrap_inner(self)->Option<U>{Arc::into_inner(self).map(T::unwrap_inner)}
}
impl<T:UnwrapInner<U>,U> UnwrapInner<Option<U>> for Option<T>{
	fn unwrap_inner(self)->Option<U>{self.map(T::unwrap_inner)}
}
/*impl<T:?Sized+Inner<U>,U:?Sized> InnerRef<U> for T{
	fn inner_ref(&self)->Self::Ref<'_>{self.inner()}
	type Ref<'a>=&'a U where Self:'a,U:'a;
}
impl<T:?Sized+InnerMut<U>,U:?Sized> InnerRefMut<U> for T{
	fn inner_ref_mut(&mut self)->Self::RefMut<'_>{self.inner_mut()}
	type RefMut<'a>=&'a mut U where Self:'a,U:'a;
}*/
#[macro_export]
/// uses inner to implement inner ref
macro_rules! direct_ref{
	(@inner $name:ident$(<$($param:ident)*>)?$(where $($bound:tt)*)?)=>(
		impl<T307EC305B4556985:?Sized,$($($param)*)?> InnerRef<T307EC305B4556985> for $name$(<$($param)*>)? where Self:Inner<T307EC305B4556985>,$($($bound)*)?{
			fn inner_ref(&self)->Self::Ref<'_>{self.inner()}
			type Ref<'a>=&'a T307EC305B4556985 where Self:'a,T307EC305B4556985:'a;
		}
	);
	(@mut $name:ident$(<$($param:ident)*>)?$(where $($bound:tt)*)?)=>(
		impl<T307EC305B4556985:?Sized,$($($param)*)?> InnerRefMut<T307EC305B4556985> for $name$(<$($param)*>)? where Self:InnerMut<T307EC305B4556985>,$($($bound)*)?{
			fn inner_ref_mut(&mut self)->Self::RefMut<'_>{self.inner_mut()}
			type RefMut<'a>=&'a mut T307EC305B4556985 where Self:'a,T307EC305B4556985:'a;
		}
	);
}
#[macro_export]
/// implement get inner, inner, inner mut, unwrap inner for a type such that it's its own inner value
macro_rules! self_inner{
	(@get $name:ident$(<$($param:ident)*>)?$(where $($bound:tt)*)?)=>(
		impl $(<$($param)*>)? GetInner<Self> for $name$(<$($param)*>)? where Self:Clone,$($($bound)*)?{
			fn get_inner(&self)->Self{self.clone()}
		}
	);
	(@inner $name:ident$(<$($param:ident)*>)?$(where $($bound:tt)*)?)=>(
		impl $(<$($param)*>)? Inner<Self> for $name$(<$($param)*>)? $(where $($bound)*)?{
			fn inner(&self)->&Self{self}
		}
	);
	(@mut $name:ident$(<$($param:ident)*>)?$(where $($bound:tt)*)?)=>(
		impl $(<$($param)*>)? InnerMut<Self> for $name$(<$($param)*>)? $(where $($bound)*)?{
			fn inner_mut(&mut self)->&mut Self{self}
		}
	);
}
/// declares function for getting a presumably owned inner value
pub trait GetInner<T>{
	/// gets the inner value
	fn get_inner(&self)->T;
}
/// declares function for getting a shared reference to the inner value
pub trait Inner<T:?Sized>{
	/// references the inner value
	fn inner(&self)->&T;
}
/// declares function for getting a mutable reference to the inner value
pub trait InnerMut<T:?Sized>{
	/// references the inner value
	fn inner_mut(&mut self)->&mut T;
}
/// declares function for getting a shared smart reference to the inner value
pub trait InnerRef<T:?Sized>{
	/// references the inner value
	fn inner_ref(&self)->Self::Ref<'_>;
	/// the reference type
	type Ref<'a> where Self:'a,T:'a;
}
/// declares function for getting a mutable smart reference to the inner value
pub trait InnerRefMut<T:?Sized>{
	/// references the inner value
	fn inner_ref_mut(&mut self)->Self::RefMut<'_>;
	/// the reference type
	type RefMut<'a> where Self:'a,T:'a;
}
/// declares function for converting into the inner value
pub trait UnwrapInner<T>{
	/// unwraps the inner value
	fn unwrap_inner(self)->T;
}
self_inner!(@get Vec<T>);
self_inner!(@get bool);
self_inner!(@get char);
self_inner!(@get f32);
self_inner!(@get f64);
self_inner!(@get i128);
self_inner!(@get i16);
self_inner!(@get i32);
self_inner!(@get i64);
self_inner!(@get i8);
self_inner!(@get isize);
self_inner!(@get u128);
self_inner!(@get u16);
self_inner!(@get u32);
self_inner!(@get u64);
self_inner!(@get u8);
self_inner!(@get usize);
self_inner!(@inner Slice<T>);
self_inner!(@inner Vec<T>);
self_inner!(@inner bool);
self_inner!(@inner char);
self_inner!(@inner f32);
self_inner!(@inner f64);
self_inner!(@inner i128);
self_inner!(@inner i16);
self_inner!(@inner i32);
self_inner!(@inner i64);
self_inner!(@inner i8);
self_inner!(@inner isize);
self_inner!(@inner u128);
self_inner!(@inner u16);
self_inner!(@inner u32);
self_inner!(@inner u64);
self_inner!(@inner u8);
self_inner!(@inner usize);
self_inner!(@mut Slice<T>);
self_inner!(@mut Vec<T>);
self_inner!(@mut bool);
self_inner!(@mut char);
self_inner!(@mut f32);
self_inner!(@mut f64);
self_inner!(@mut i128);
self_inner!(@mut i16);
self_inner!(@mut i32);
self_inner!(@mut i64);
self_inner!(@mut i8);
self_inner!(@mut isize);
self_inner!(@mut u128);
self_inner!(@mut u16);
self_inner!(@mut u32);
self_inner!(@mut u64);
self_inner!(@mut u8);
self_inner!(@mut usize);
type Slice<T>=[T];
pub use {direct_ref,self_inner};
use std::sync::Arc;
