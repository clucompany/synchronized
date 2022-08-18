
//! The core of the library that defines the basic primitives.

use core::marker::PhantomData;

use core::ops::Deref;
use core::ops::DerefMut;

/// Implementation of the behavior for the used synchronization structure.
pub trait SyncPointBeh {
	/// The actual structure that holds the synchronization 
	/// and provides access to the data.
	type LockType: Deref<Target = Self::DerefLockType> + DerefMut;
	
	/// The data type to modify, provided by the synchronization structure.
	type DerefLockType;
	
	/// Create a new hold lock.
	fn new_lock(&self) -> Self::LockType;
	
	/// Whether the current lock is active
	#[cfg_attr(docsrs, doc(cfg(feature = "parking_lot")))]
	#[cfg( feature = "parking_lot" )]
	fn is_lock(&self) -> bool;
	
	/// If the lock exists and is not released, then return None, 
	/// if there is no lock, then create it and return Some.
	#[cfg_attr(docsrs, doc(cfg( any( feature = "parking_lot", feature = "std" ) )))]
	#[cfg( any( feature = "parking_lot", feature = "std" ) )]
	fn try_lock(&self) -> Option<Self::LockType>;
	
	/// Destroy the blocking structure and remove 
	/// the lock (usually always involves just a drop)
	fn unlock(&self, lock_type: Self::LockType);
}

#[cfg_attr(docsrs, doc(cfg(feature = "get_point_name")))]
#[cfg( feature = "get_point_name" )]
/// Determining the Synchronization Point Name
pub trait SyncPointName {
	/// Synchronization point name
	const NAME: &'static str;
	
	/// Getting the sync point name
	#[inline(always)]
	fn get_name() -> &'static str {
		Self::NAME
	}
}

#[cfg_attr(docsrs, doc(cfg(feature = "get_point_name")))]
#[cfg( feature = "get_point_name" )]
impl SyncPointName for () {
	const NAME: &'static str = "<empty>";
}

/// Universal synchronization point structure, 
/// combining various types of locks and working with them.
#[repr(transparent)]
pub struct SyncPoint<T, N> {
	/// Generalized structure for generating locks.
	mutex_builder: T,
	
	/// The phantom used to implement `SyncPointName`.
	phantom_name: PhantomData<N>,
}

impl<T, N> SyncPoint<T, N> where T: SyncPointBeh {
	/// Structure creation.
	#[inline(always)]
	pub const fn new(mutex_builder: T) -> Self {
		Self {
			mutex_builder,
			
			phantom_name: PhantomData,
		}
	}
	
	/// Create a new hold lock.
	#[inline(always)]
	pub fn new_lock(&self) -> T::LockType {
		T::new_lock(&self.mutex_builder)
	}
	
	/// Destroy the blocking structure and remove the lock 
	/// (usually always involves just a drop).
	#[inline(always)]
	pub fn unlock(&self, lock: T::LockType) {
		T::unlock(&self.mutex_builder, lock);
	}
	
	/// Whether the current lock is active
	#[inline(always)]
	#[cfg_attr(docsrs, doc(cfg(feature = "parking_lot")))]
	#[cfg( feature = "parking_lot" )]
	pub fn is_lock(&self) -> bool {
		T::is_lock(&self.mutex_builder)
	}
	
	/// If the lock exists and is not released, then return None, 
	/// if there is no lock, then create it and return Some.
	#[inline(always)]
	#[cfg_attr(docsrs, doc(cfg( any( feature = "parking_lot", feature = "std" ) )))]
	#[cfg( any( feature = "parking_lot", feature = "std" ) )]
	pub fn try_lock(&self) -> Option<T::LockType> {
		T::try_lock(&self.mutex_builder)
	}
}

#[cfg_attr(docsrs, doc(cfg(feature = "get_point_name")))]
#[cfg( feature = "get_point_name" )]
impl<T, N> SyncPoint<T, N> where N: SyncPointName {
	/// Getting the sync point name.
	#[inline(always)]
	pub const fn get_sync_point_name(&self) -> &'static str {
		N::NAME
	}
	
	/// Getting the sync point name.
	#[inline(always)]
	pub const fn get_name() -> &'static str {
		N::NAME
	}
}

#[cfg( not(feature = "get_point_name") )]
impl<T, N> SyncPoint<T, N> {
	/// Getting the sync point name
	/// 
	/// Warning since `get_point_name` is disabled, 
	/// "<unknown>" will always be returned.
	#[inline(always)]
	pub const fn get_sync_point_name(&self) -> &'static str {
		"<unknown>"
	}
	
	/// Getting the sync point name
	/// 
	/// Warning since `get_point_name` is disabled, 
	/// "<unknown>" will always be returned.
	#[inline(always)]
	pub const fn get_name() -> &'static str {
		"<unknown>"
	}
}

/// Generic macro that generates and implements a `SyncPointName`.
#[cfg_attr(docsrs, doc(cfg(feature = "get_point_name")))]
#[cfg( feature = "get_point_name" )]
#[macro_export]
#[doc(hidden)]
macro_rules! __make_name {
	[ #new_name<$name:ident>: $expr:expr $(; $($unk:tt)*)? ] => {
		/// An automatically generated enum for 
		/// the type-based implementation of SyncPointName.
		pub enum $name {}
		
		impl $crate::core::SyncPointName for $name {
			const NAME: &'static str = $expr;
		}
		
		$(
			$crate::__make_name! {
				$($unk)*
			}
		)?
	};
	
	[ #get_name<$name:ident> ] => { $name };
	
	[] => {}
}

/// Generic macro that generates and implements a `SyncPointName`.
#[cfg( not(feature = "get_point_name") )]
#[cfg_attr(docsrs, doc(cfg(not(feature = "get_point_name"))))]
#[macro_export]
#[doc(hidden)]
macro_rules! __make_name {
	[
		// The `get_point_name` function is disabled, 
		// only a stub is used.
		#new_name<$name:ident>: $expr:expr $(; $($unk:tt)*)?
	] => {};
	
	[
		// The `get_point_name` function is disabled, 
		// only a stub is used.
		#get_name<$name:ident>
	] => {
		()
	};
	
	[] => {}
}
