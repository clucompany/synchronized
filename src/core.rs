
pub trait BehSyncPoint {
	type LockType;
	
	fn lock(&self) -> Self::LockType;
	fn is_lock(&self) -> bool;
	fn try_lock(&self) -> Option<Self::LockType>;
	fn unlock(&self, lock_type: Self::LockType);
}

#[repr(transparent)]
pub struct SyncPoint<T> {
	sync_point: T
}

impl<T> SyncPoint<T> {
	#[inline(always)]
	pub const fn const_new(sync_point: T) -> SyncPoint<T> {
		SyncPoint {
			sync_point
		}
	}
}

impl<T> SyncPoint<T> where T: BehSyncPoint {
	#[inline(always)]
	pub fn new(sync_point: T) -> Self {
		Self {
			sync_point
		}
	}
	
	#[inline(always)]
	pub fn lock(&self) -> T::LockType {
		T::lock(&self.sync_point)
	}
	
	#[inline(always)]
	pub fn unlock(&self, lock: T::LockType) {
		T::unlock(&self.sync_point, lock);
	}
	
	#[inline(always)]
	pub fn is_lock(&self) -> bool {
		T::is_lock(&self.sync_point)
	}
	
	#[inline(always)]
	pub fn try_lock(&self) -> Option<T::LockType> {
		T::try_lock(&self.sync_point)
	}
}
