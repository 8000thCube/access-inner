impl<T:?Sized> Unified for T{}
/// functions for getting or referencing values
pub mod access;
/// wrappers for controlling where access or unwrapping stops
pub mod wrap;
/// provided internal data access and wrapping functions, unified into one trait for convenience on import and calls on generic trait funtions
pub trait Unified{
	/// gets the inner value
	fn get<T>(&self)->T where Self:GetInner<T>{self.get_inner()}
	/// references the inner value
	fn inner<T:?Sized>(&self)->&T where Self:Inner<T>{Inner::inner(self)}
	/// references the inner value
	fn inner_mut<T:?Sized>(&mut self)->&mut T where Self:InnerMut<T>{InnerMut::inner_mut(self)}
	/// references the inner value
	fn inner_ref<T:?Sized>(&self)-><Self as InnerRef<T>>::Ref<'_> where Self:InnerRef<T>{InnerRef::inner_ref(self)}
	/// references the inner value
	fn inner_ref_mut<T:?Sized>(&mut self)-><Self as InnerRefMut<T>>::RefMut<'_> where Self:InnerRefMut<T>{InnerRefMut::inner_ref_mut(self)}
	/// unwraps the inner value
	fn unwrap_inner<T>(self)->T where Self:Sized+UnwrapInner<T>{UnwrapInner::unwrap_inner(self)}
	/// wraps so that access and unwrapping operations stop here
	fn wrap_inner(self)->WrappedInner<Self> where Self:Sized{WrappedInner(self)}
	/// wraps so that access and unwrapping operations stop here
	fn wrap_inner_mut(&mut self)->&mut WrappedInner<Self>{
		unsafe{mem::transmute(self)}		// transmuting a transparent reference
	}
	/// wraps so that access and unwrapping operations stop here
	fn wrap_inner_ref(&self)->&WrappedInner<Self>{
		unsafe{mem::transmute(self)}		// transmuting a transparent reference
	}
}
use {access::*,wrap::*};
use core::mem;
