use crate::prelude::*;
use crate::gfx::*;

use slotmap::SlotMap;
use std::rc::{Rc, Weak};
use std::cell::{Cell, UnsafeCell};

slotmap::new_key_type!{
	pub struct TextureKey;
	pub struct FramebufferKey;
}


#[derive(Debug)]
pub struct Resources {
	inner: Rc<ResourcesRefCell>,
}

impl Resources {
	pub(super) fn new() -> Resources {
		Resources { inner: ResourcesRefCell::new().into() }
	}

	pub(super) fn insert_texture(&mut self, texture: Texture) -> TextureKey {
		self.inner.mutate(move |inner| inner.textures.insert((texture, BorrowState::new())))
	}

	pub(super) fn insert_framebuffer(&mut self, framebuffer: Framebuffer) -> FramebufferKey {
		self.inner.mutate(move |inner| inner.framebuffers.insert((framebuffer, BorrowState::new())))
	}

	pub(super) fn on_resize(&mut self, backbuffer_size: Vec2i) {
		// TODO(pat.m): how to not do this
		let mut inner = self.inner.borrow_mut();

		for (texture, _) in inner.textures.values_mut() {
			texture.on_resize(backbuffer_size);
		}

		// HACK: this is v gross
		drop(inner);
		let inner = self.inner.borrow();

		for (framebuffer, _) in inner.framebuffers.values() {
			framebuffer.rebind_attachments(&self);
		}
	}

	pub fn get<H, R>(&self, handle: H) -> ResourceLock<R>
		where H: ResourceKey<Resource=R>
	{
		handle.get(self)
	}

	pub fn get_mut<H, R>(&mut self, handle: H) -> ResourceLockMut<R>
		where H: ResourceKey<Resource=R>
	{
		handle.get_mut(self)
	}
}




pub trait ResourceKey {
	type Resource;

	fn get(&self, _: &Resources) -> ResourceLock<Self::Resource>;
	fn get_mut(&self, _: &mut Resources) -> ResourceLockMut<Self::Resource>;
}


impl ResourceKey for TextureKey {
	type Resource = Texture;

	fn get(&self, resources: &Resources) -> ResourceLock<Self::Resource> {
		resources.inner.borrow_resource(move |inner| &inner.textures[*self])
	}

	fn get_mut(&self, resources: &mut Resources) -> ResourceLockMut<Self::Resource> {
		resources.inner.borrow_resource_mut(move |inner| &mut inner.textures[*self])
	}
}


impl ResourceKey for FramebufferKey {
	type Resource = Framebuffer;

	fn get(&self, resources: &Resources) -> ResourceLock<Self::Resource> {
		resources.inner.borrow_resource(move |inner| &inner.framebuffers[*self])
	}

	fn get_mut(&self, resources: &mut Resources) -> ResourceLockMut<Self::Resource> {
		resources.inner.borrow_resource_mut(move |inner| &mut inner.framebuffers[*self])
	}
}




// See std::cell::RefCell
#[derive(Debug)]
struct ResourcesRefCell {
	inner_borrow_state: BorrowState,
	inner: UnsafeCell<ResourcesInner>,
}

#[derive(Debug)]
struct ResourcesInner {
	textures: SlotMap<TextureKey, (Texture, BorrowState)>,
	framebuffers: SlotMap<FramebufferKey, (Framebuffer, BorrowState)>,
}

impl ResourcesRefCell {
	fn new() -> ResourcesRefCell {
		ResourcesRefCell {
			inner_borrow_state: BorrowState::new(),
			inner: UnsafeCell::new(
				ResourcesInner {
					textures: SlotMap::with_key(),
					framebuffers: SlotMap::with_key(),
				}
			)
		}
	}

	#[deprecated]
	fn borrow(self: &Rc<ResourcesRefCell>) -> ResourceLock<ResourcesInner> {
		self.inner_borrow_state.borrow();
		ResourceLock {
			inner: Rc::clone(self),
			resource: self.inner.get(),
			resource_borrow_state: std::ptr::null(),
		}
	}

	#[deprecated]
	fn borrow_mut(self: &Rc<ResourcesRefCell>) -> ResourceLockMut<ResourcesInner> {
		self.inner_borrow_state.borrow_mut();
		ResourceLockMut {
			inner: Rc::clone(self),
			resource: self.inner.get(),
			resource_borrow_state: std::ptr::null(),
		}
	}


	fn mutate<F, R>(&self, f: F) -> R
		where F: FnOnce(&mut ResourcesInner) -> R
	{
		// Mark ResourceInner as being mutably borrowed for the duration of function call
		self.inner_borrow_state.borrow_mut();

		// TODO(pat.m): FIGURE OUT IF IT IS ACTUALLY SOUND TO RETURN STUFF HERE
		// Its only for internal use so _should_ be fine, but even so
		let result = f(unsafe { &mut *self.inner.get() });

		self.inner_borrow_state.unborrow_mut();

		result
	}

	fn borrow_resource<F, R>(self: &Rc<ResourcesRefCell>, f: F) -> ResourceLock<R>
		where F: FnOnce(&ResourcesInner) -> &(R, BorrowState)
	{
		// Mark ResourceInner as being borrowed - no changes to the collections can be made from this point
		self.inner_borrow_state.borrow();

		// Now that we've locked inner for read, we can safely dereference it, and get a reference to the resource
		let inner = unsafe { &*self.inner.get() };
		let (resource, resource_borrow_state) = f(inner);

		// Mark resource itself as being borrowed
		resource_borrow_state.borrow();

		ResourceLock {
			inner: Rc::clone(self),
			resource,
			resource_borrow_state,
		}
	}

	fn borrow_resource_mut<F, R>(self: &Rc<ResourcesRefCell>, f: F) -> ResourceLockMut<R>
		where F: FnOnce(&mut ResourcesInner) -> &mut (R, BorrowState)
	{
		// Mark ResourceInner as being borrowed - no changes to the collections can be made from this point
		// NOTE: mutable borrows of resources only immutably borrow ResourceInner - since individual resources
		// track their own borrow state, and we only need to guarantee that those resources stay pinned while borrowed
		self.inner_borrow_state.borrow();

		// Now that we've locked inner for read, we can safely dereference it, and get a reference to the resource.
		// NOTE: we are making the assumption that `f` DOES NOT modify the storage of resources, and only creates a
		// new reference into the storage. anything else would probably cause UB.
		// We are also assuming that there is no reentrancy, since that could create aliasing
		// mutable references, which would also be UB.
		let inner = unsafe { &mut *self.inner.get() };
		let (resource, resource_borrow_state) = f(inner);

		// Mark resource itself as being borrowed
		resource_borrow_state.borrow_mut();

		ResourceLockMut {
			inner: Rc::clone(self),
			resource,
			resource_borrow_state,
		}
	}
}


pub struct ResourceLock<T> {
	inner: Rc<ResourcesRefCell>,
	resource: *const T,
	resource_borrow_state: *const BorrowState, // Can be null
}

impl<T> std::ops::Deref for ResourceLock<T> {
	type Target = T;

	fn deref(&self) -> &'_ T {
		unsafe { &*self.resource }
	}
}

impl<T> Drop for ResourceLock<T> {
	fn drop(&mut self) {
		// Unborrow resource if it is individually locked
		if let Some(resource_borrow_state) = unsafe{self.resource_borrow_state.as_ref()} {
			resource_borrow_state.unborrow();
		}

		self.inner.inner_borrow_state.unborrow();
	}
}



pub struct ResourceLockMut<T> {
	inner: Rc<ResourcesRefCell>,
	resource: *mut T,
	resource_borrow_state: *const BorrowState, // Can be null
}

impl<T> std::ops::Deref for ResourceLockMut<T> {
	type Target = T;

	fn deref(&self) -> &'_ T {
		unsafe { &*self.resource }
	}
}

impl<T> std::ops::DerefMut for ResourceLockMut<T> {
	fn deref_mut(&mut self) -> &'_ mut T {
		unsafe { &mut *self.resource }
	}
}

impl<T> Drop for ResourceLockMut<T> {
	fn drop(&mut self) {
		// Unborrow resource if it is individually locked
		if let Some(resource_borrow_state) = unsafe{self.resource_borrow_state.as_ref()} {
			resource_borrow_state.unborrow_mut();

			// NOTE: mutable borrows of resources only immutably borrow ResourceInner
			self.inner.inner_borrow_state.unborrow();
		} else {
			self.inner.inner_borrow_state.unborrow_mut();
		}
	}
}



#[derive(Debug)]
struct BorrowState(Cell<isize>);

impl BorrowState {
	fn new() -> Self {
		BorrowState(Cell::new(0))
	}

	fn borrow(&self) {
		let new_borrow_state = self.0.get() + 1;
		assert!(new_borrow_state > 0, "tried to immutably borrow while already mutably borrowed");
		self.0.set(new_borrow_state);
	}

	fn unborrow(&self) {
		let new_borrow_state = self.0.get() - 1;
		assert!(new_borrow_state >= 0);
		self.0.set(new_borrow_state);
	}

	fn borrow_mut(&self) {
		assert!(self.0.get() == 0, "tried to mutably borrow while already borrowed");
		self.0.set(-1);
	}

	fn unborrow_mut(&self) {
		assert!(self.0.get() == -1);
		self.0.set(0);
	}
}


// #[derive(Clone, Debug)]
// pub struct ResourceHandle<KeyType, ResourceType> {
// 	inner: Weak<ResourcesRefCell>,
// 	key: KeyType,
// 	_resource: std::marker::PhantomData<*const ResourceType>,
// }

// impl<KeyType: Copy, ResourceType> ResourceHandle<KeyType, ResourceType> {
// 	fn new(key: KeyType, inner: Weak<ResourcesRefCell>) -> Self {
// 		ResourceHandle {
// 			inner,
// 			key,
// 			_resource: std::marker::PhantomData,
// 		}
// 	}

// 	pub fn key(&self) -> KeyType { self.key }
// }




// pub type TextureHandle = ResourceHandle<TextureKey, Texture>;

// impl TextureHandle {
// 	pub fn lock_mut(&self) -> ResourceLockMut<Texture> {
// 		let inner = Weak::upgrade(&self.inner)
// 			.expect("Failed to lock gfx resources");

// 		let key = self.key;
// 		inner.borrow_mut()
// 			.project(move |resources| &mut resources.textures[key])
// 	}

// 	pub fn lock(&self) -> ResourceLock<Texture> {
// 		let inner = Weak::upgrade(&self.inner)
// 			.expect("Failed to lock gfx resources");

// 		let key = self.key;
// 		inner.borrow()
// 			.project(move |resources| &resources.textures[key])
// 	}
// }



// pub type FramebufferHandle = ResourceHandle<FramebufferKey, Framebuffer>;

// impl FramebufferHandle {
// 	pub fn lock_mut(&self) -> ResourceLockMut<Framebuffer> {
// 		let inner = Weak::upgrade(&self.inner)
// 			.expect("Failed to lock gfx resources");

// 		let key = self.key;
// 		inner.borrow_mut()
// 			.project(move |resources| &mut resources.framebuffers[key])
// 	}

// 	pub fn lock(&self) -> ResourceLock<Framebuffer> {
// 		let inner = Weak::upgrade(&self.inner)
// 			.expect("Failed to lock gfx resources");

// 		let key = self.key;
// 		inner.borrow()
// 			.project(move |resources| &resources.framebuffers[key])
// 	}
// }

