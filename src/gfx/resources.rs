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
pub(super) struct Resources {
	inner: Rc<ResourcesRefCell>,
}

impl Resources {
	pub(super) fn new() -> Resources {
		Resources { inner: ResourcesRefCell::new().into() }
	}

	pub(super) fn insert_texture(&mut self, texture: Texture) -> TextureKey {
		self.inner.borrow_mut().textures.insert(texture)
	}

	pub(super) fn insert_framebuffer(&mut self, texture: Framebuffer) -> FramebufferKey {
		self.inner.borrow_mut().framebuffers.insert(texture)
	}

	pub(super) fn on_resize(&mut self, backbuffer_size: Vec2i) {
		let mut inner = self.inner.borrow_mut();

		for texture in inner.textures.values_mut() {
			texture.on_resize(backbuffer_size);
		}

		// HACK: this is v gross
		drop(inner);
		let inner = self.inner.borrow();

		for framebuffer in inner.framebuffers.values() {
			framebuffer.rebind_attachments(&self);
		}
	}

	pub(super) fn get<H, R>(&self, handle: H) -> ResourceLock<R>
		where H: ResourceKey<Resource=R>
	{
		handle.get(&self)
	}

	pub(super) fn get_mut<H, R>(&mut self, handle: H) -> ResourceLockMut<R>
		where H: ResourceKey<Resource=R>
	{
		handle.get_mut(&self)
	}
}




pub(super) trait ResourceKey {
	type Resource;

	fn get(&self, _: &Resources) -> ResourceLock<Self::Resource>;
	fn get_mut(&self, _: &Resources) -> ResourceLockMut<Self::Resource>;
}


impl ResourceKey for TextureKey {
	type Resource = Texture;

	fn get(&self, resources: &Resources) -> ResourceLock<Self::Resource> {
		resources.inner.borrow()
			.project(move |inner| &inner.textures[*self])

	}

	fn get_mut(&self, resources: &Resources) -> ResourceLockMut<Self::Resource> {
		resources.inner.borrow_mut()
			.project(move |inner| &mut inner.textures[*self])
	}
}


impl ResourceKey for FramebufferKey {
	type Resource = Framebuffer;

	fn get(&self, resources: &Resources) -> ResourceLock<Self::Resource> {
		resources.inner.borrow()
			.project(move |inner| &inner.framebuffers[*self])

	}

	fn get_mut(&self, resources: &Resources) -> ResourceLockMut<Self::Resource> {
		resources.inner.borrow_mut()
			.project(move |inner| &mut inner.framebuffers[*self])
	}
}




// NOTE:
// Maybe instead of all of this nonsense, Framebuffers just own Textures and are responsible for resizing them?
// or maybe the Context manages a list of specific backbuffer textures?


// See std::cell::RefCell
#[derive(Debug)]
struct ResourcesRefCell {
	borrow_state: Cell<isize>,
	inner: UnsafeCell<ResourcesInner>,
}

#[derive(Debug)]
struct ResourcesInner {
	textures: SlotMap<TextureKey, Texture>,
	framebuffers: SlotMap<FramebufferKey, Framebuffer>,
}

impl ResourcesRefCell {
	fn new() -> ResourcesRefCell {
		ResourcesRefCell {
			borrow_state: Cell::default(),
			inner: UnsafeCell::new(
				ResourcesInner {
					textures: SlotMap::with_key(),
					framebuffers: SlotMap::with_key(),
				}
			)
		}
	}

	fn borrow(self: &Rc<ResourcesRefCell>) -> ResourceLock<ResourcesInner> {
		let b = self.borrow_state.get().wrapping_add(1);

		// If borrow_state previously signalled mutable borrow - fail
		if b <= 0 {
			panic!("Failed to borrow")
		} else {
			self.borrow_state.set(b);
			ResourceLock {
				inner: Rc::clone(self),
				reference: self.inner.get(),
			}
		}
	}

	fn borrow_mut(self: &Rc<ResourcesRefCell>) -> ResourceLockMut<ResourcesInner> {
		// see BorrowMutRef
		match self.borrow_state.get() {
			0 => {
				self.borrow_state.set(-1);
				ResourceLockMut {
					inner: Rc::clone(self),
					reference: self.inner.get(),
				}
			}

			_ => panic!("ResourcesRefCell already mutably borrowed")
		}
	}
}


pub struct ResourceLock<T> {
	inner: Rc<ResourcesRefCell>,
	reference: *const T,
}

impl<T> ResourceLock<T> {
	fn project<U, F>(self, f: F) -> ResourceLock<U>
		where F: FnOnce(&T) -> &U
	{
		let inner = self.inner.clone();

		// Add one to the shared ref count so that when the original lock drops, we stay locked
		let borrow_state = inner.borrow_state.get();
		assert!(borrow_state > 0);
		assert!(borrow_state != isize::MAX);
		inner.borrow_state.set(borrow_state+1);

		ResourceLock {
			inner,
			reference: f(unsafe {&*self.reference}),
		}
	}
}

impl<T> std::ops::Deref for ResourceLock<T> {
	type Target = T;

	fn deref(&self) -> &'_ T {
		unsafe { &*self.reference }
	}
}

impl<T> Drop for ResourceLock<T> {
	fn drop(&mut self) {
		let borrow = self.inner.borrow_state.get();
		assert!(borrow > 0);
		self.inner.borrow_state.set(borrow - 1);
	}
}



pub struct ResourceLockMut<T> {
	inner: Rc<ResourcesRefCell>,
	reference: *mut T,
}

impl<T> ResourceLockMut<T> {
	fn project<U, F>(self, f: F) -> ResourceLockMut<U>
		where F: FnOnce(&mut T) -> &mut U
	{
		let inner = self.inner.clone();

		// Add one to the mutable ref count so that when the original lock drops, we stay locked
		let borrow_state = inner.borrow_state.get();
		assert!(borrow_state < 0);
		assert!(borrow_state != isize::MIN);
		inner.borrow_state.set(borrow_state-1);

		ResourceLockMut {
			inner,
			reference: f(unsafe {&mut *self.reference}),
		}
	}
}

impl<T> std::ops::Deref for ResourceLockMut<T> {
	type Target = T;

	fn deref(&self) -> &'_ T {
		unsafe { &*self.reference }
	}
}

impl<T> std::ops::DerefMut for ResourceLockMut<T> {
	fn deref_mut(&mut self) -> &'_ mut T {
		unsafe { &mut *self.reference }
	}
}

impl<T> Drop for ResourceLockMut<T> {
	fn drop(&mut self) {
		let borrow = self.inner.borrow_state.get();
		assert!(borrow < 0);
		self.inner.borrow_state.set(borrow + 1);
	}
}





#[derive(Clone, Debug)]
pub struct ResourceHandle<KeyType, ResourceType> {
	inner: Weak<ResourcesRefCell>,
	key: KeyType,
	_resource: std::marker::PhantomData<*const ResourceType>,
}

impl<KeyType: Copy, ResourceType> ResourceHandle<KeyType, ResourceType> {
	fn new(key: KeyType, inner: Weak<ResourcesRefCell>) -> Self {
		ResourceHandle {
			inner,
			key,
			_resource: std::marker::PhantomData,
		}
	}

	pub fn key(&self) -> KeyType { self.key }
}




pub type TextureHandle = ResourceHandle<TextureKey, Texture>;

impl TextureHandle {
	pub fn lock_mut(&self) -> ResourceLockMut<Texture> {
		let inner = Weak::upgrade(&self.inner)
			.expect("Failed to lock gfx resources");

		let key = self.key;
		inner.borrow_mut()
			.project(move |resources| &mut resources.textures[key])
	}

	pub fn lock(&self) -> ResourceLock<Texture> {
		let inner = Weak::upgrade(&self.inner)
			.expect("Failed to lock gfx resources");

		let key = self.key;
		inner.borrow()
			.project(move |resources| &resources.textures[key])
	}
}



pub type FramebufferHandle = ResourceHandle<FramebufferKey, Framebuffer>;

impl FramebufferHandle {
	pub fn lock_mut(&self) -> ResourceLockMut<Framebuffer> {
		let inner = Weak::upgrade(&self.inner)
			.expect("Failed to lock gfx resources");

		let key = self.key;
		inner.borrow_mut()
			.project(move |resources| &mut resources.framebuffers[key])
	}

	pub fn lock(&self) -> ResourceLock<Framebuffer> {
		let inner = Weak::upgrade(&self.inner)
			.expect("Failed to lock gfx resources");

		let key = self.key;
		inner.borrow()
			.project(move |resources| &resources.framebuffers[key])
	}
}

