impl<T:GetInner<U>,U> GetInner<Option<U>> for Option<T>{
	fn get_inner(&self)->Option<U>{self.as_ref().map(T::get_inner)}
}
impl<T:GetInner<U>,U> GetInner<U> for Arc<T>{
	fn get_inner(&self)->U{(**self).get_inner()}
}
impl<T:Inner<U>,U> Inner<U> for Arc<T>{
	fn inner(&self)->&U{(**self).inner()}
}
impl<T:InnerRef<U>,U> InnerRef<Option<U>> for Option<T>{
	fn inner_ref(&self)->Self::Ref<'_>{
		match self{
			None=>None,
			Some(t)=>Some(t.inner_ref())
		}
	}
	type Ref<'a>=Option<T::Ref<'a>> where T:'a,U:'a;
}
impl<T:InnerRefMut<U>,U> InnerRefMut<Option<U>> for Arc<T>{
	fn inner_ref_mut(&mut self)->Self::RefMut<'_>{
		match Arc::get_mut(self){
			None=>None,
			Some(t)=>Some(t.inner_ref_mut())
		}
	}
	type RefMut<'a>=Option<T::RefMut<'a>> where T:'a,U:'a;
}
impl<T:InnerRefMut<U>,U> InnerRefMut<Option<U>> for Option<T>{
	fn inner_ref_mut(&mut self)->Self::RefMut<'_>{
		match self{
			None=>None,
			Some(t)=>Some(t.inner_ref_mut())
		}
	}
	type RefMut<'a>=Option<T::RefMut<'a>> where T:'a,U:'a;
}
impl<T:UnwrapInner<U>,U> UnwrapInner<Option<U>> for Option<T>{
	fn unwrap_inner(self)->Option<U>{self.map(T::unwrap_inner)}
}
impl<T:?Sized+Inner<U>,U:?Sized> InnerRef<U> for T{
	fn inner_ref(&self)->Self::Ref<'_>{self.inner()}
	type Ref<'a>=&'a U where Self:'a,U:'a;
}
impl<T:?Sized+InnerMut<U>,U:?Sized> InnerRefMut<U> for T{
	fn inner_ref_mut(&mut self)->Self::RefMut<'_>{self.inner_mut()}
	type RefMut<'a>=&'a mut U where Self:'a,U:'a;
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
use std::sync::Arc;
