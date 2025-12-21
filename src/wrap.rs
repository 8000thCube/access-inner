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
#[derive(Clone,Copy,Debug,Default,Eq,Hash,Ord,PartialEq,PartialOrd)]
#[repr(transparent)]
/// wrapper to make unwrap_inner and other access functions that might just call versions of themselves on the inner value stop here
pub struct WrappedInner<T:?Sized>(pub T);
use crate::access::*;
