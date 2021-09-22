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
		self.inner.mutate(move |inner| inner.textures.insert((texture, Cell::default())))
	}

	pub(super) fn insert_framebuffer(&mut self, framebuffer: Framebuffer) -> FramebufferKey {
		self.inner.mutate(move |inner| inner.framebuffers.insert((framebuffer, Cell::default())))
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
	inner_borrow_state: Cell<isize>,
	inner: UnsafeCell<ResourcesInner>,
}

#[derive(Debug)]
struct ResourcesInner {
	textures: SlotMap<TextureKey, (Texture, Cell<isize>)>,
	framebuffers: SlotMap<FramebufferKey, (Framebuffer, Cell<isize>)>,
}

impl ResourcesRefCell {
	fn new() -> ResourcesRefCell {
		ResourcesRefCell {
			inner_borrow_state: Cell::default(),
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
		let b = self.inner_borrow_state.get().wrapping_add(1);

		// If inner_borrow_state previously signalled mutable borrow - fail
		if b <= 0 {
			panic!("Failed to borrow")
		} else {
			self.inner_borrow_state.set(b);
			ResourceLock {
				inner: Rc::clone(self),
				resource: self.inner.get(),
				resource_borrow_state: std::ptr::null(),
			}
		}
	}

	#[deprecated]
	fn borrow_mut(self: &Rc<ResourcesRefCell>) -> ResourceLockMut<ResourcesInner> {
		// see BorrowMutRef
		match self.inner_borrow_state.get() {
			0 => {
				self.inner_borrow_state.set(-1);
				ResourceLockMut {
					inner: Rc::clone(self),
					resource: self.inner.get(),
					resource_borrow_state: std::ptr::null(),
				}
			}

			_ => panic!("ResourcesRefCell already mutably borrowed")
		}
	}


	fn mutate<F, R>(&self, f: F) -> R
		where F: FnOnce(&mut ResourcesInner) -> R
	{
		// Mark ResourceInner as being mutably borrowed for the duration of function call
		assert!(self.inner_borrow_state.get() == 0, "tried to mutably borrow ResourceInner while already borrowed mutably");
		self.inner_borrow_state.set(-1);

		// TODO(pat.m): FIGURE OUT IF IT IS ACTUALLY SOUND TO RETURN STUFF HERE
		// Its only for internal use so _should_ be fine, but even so
		let result = f(unsafe { &mut *self.inner.get() });

		assert!(self.inner_borrow_state.get() == -1);
		self.inner_borrow_state.set(0);

		result
	}

	fn borrow_resource<F, R>(self: &Rc<ResourcesRefCell>, f: F) -> ResourceLock<R>
		where F: FnOnce(&ResourcesInner) -> &(R, Cell<isize>)
	{
		// Mark ResourceInner as being borrowed - no changes to the collections can be made from this point
		let new_total_borrow_state = self.inner_borrow_state.get() + 1;
		if new_total_borrow_state <= 0 {
			panic!("tried to borrow resource while ResourceInner borrowed mutably");
		}

		self.inner_borrow_state.set(new_total_borrow_state);

		// Now that we've locked inner for read, we can safely dereference it, and get a reference to the resource
		let inner = unsafe { &*self.inner.get() };
		let (resource, resource_borrow_state) = f(inner);

		// Mark resource itself as being borrowed
		let new_resource_borrow_state = resource_borrow_state.get() + 1;
		if new_resource_borrow_state <= 0 {
			panic!("tried to immutably borrow resource while already mutably borrowed");
		}

		resource_borrow_state.set(new_resource_borrow_state);

		ResourceLock {
			inner: Rc::clone(self),
			resource,
			resource_borrow_state,
		}
	}

	fn borrow_resource_mut<F, R>(self: &Rc<ResourcesRefCell>, f: F) -> ResourceLockMut<R>
		where F: FnOnce(&mut ResourcesInner) -> &mut (R, Cell<isize>)
	{
		// Mark ResourceInner as being borrowed - no changes to the collections can be made from this point
		// NOTE: mutable borrows of resources only immutably borrow ResourceInner - since individual resources
		// track their own borrow state, and we only need to guarantee that those resources stay pinned while borrowed
		let new_total_borrow_state = self.inner_borrow_state.get() + 1;
		if new_total_borrow_state <= 0 {
			panic!("tried to borrow resource while ResourceInner borrowed mutably");
		}

		self.inner_borrow_state.set(new_total_borrow_state);

		// Now that we've locked inner for read, we can safely dereference it, and get a reference to the resource.
		// NOTE: we are making the assumption that `f` DOES NOT modify the storage of resources, and only creates a
		// new reference into the storage. anything else would probably cause UB.
		// We are also assuming that there is no reentrancy, since that could create aliasing
		// mutable references, which would also be UB.
		let inner = unsafe { &mut *self.inner.get() };
		let (resource, resource_borrow_state) = f(inner);

		// Mark resource itself as being borrowed
		let new_resource_borrow_state = resource_borrow_state.get() - 1;
		if new_resource_borrow_state >= 0 {
			panic!("tried to mutable borrow resource while already mutably borrowed");
		}

		resource_borrow_state.set(-1);

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
	resource_borrow_state: *const Cell<isize>, // Can be null
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
			let borrow = resource_borrow_state.get();
			assert!(borrow > 0);
			resource_borrow_state.set(borrow - 1);
		}

		// Unborrow ResourceInner
		let borrow = self.inner.inner_borrow_state.get();
		assert!(borrow > 0);
		self.inner.inner_borrow_state.set(borrow - 1);
	}
}



pub struct ResourceLockMut<T> {
	inner: Rc<ResourcesRefCell>,
	resource: *mut T,
	resource_borrow_state: *const Cell<isize>, // Can be null
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
			let borrow = resource_borrow_state.get();
			assert!(borrow < 0);
			resource_borrow_state.set(borrow + 1);

			// Unborrow ResourceInner
			// NOTE: mutable borrows of resources only immutably borrow ResourceInner
			let borrow = self.inner.inner_borrow_state.get();
			assert!(borrow > 0);
			self.inner.inner_borrow_state.set(borrow - 1);
		} else {
			// Unborrow ResourceInner
			let borrow = self.inner.inner_borrow_state.get();
			assert!(borrow < 0);
			self.inner.inner_borrow_state.set(borrow + 1);
		}
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

