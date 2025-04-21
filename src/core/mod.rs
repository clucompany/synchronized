//! The core of the library that defines the basic primitives.

pub(crate) mod r#async;

use r#async::cfg_async_or_sync;
use core::ops::Deref;
use core::ops::DerefMut;

use crate::cfg::cfg_async;
use crate::cfg::cfg_not_async;

cfg_async_or_sync! {
	/// Implementation of the behavior for the used synchronization structure.
	pub trait SyncPointBeh {
		/// This section of code is connected only if
		/// the current library is asynchronous.
		#only_async {
			/// Create a new hold lock.
			fn new_lock(&self) -> impl core::future::Future<Output = Self::LockType<'_>> + Send;
		}
		/// This section of code is connected only if
		/// the current library is synchronous.
		#only_sync {
			/// Create a new hold lock.
			fn new_lock(&self) -> Self::LockType<'_>;
		}

		/// If the lock exists and is not released, then return None,
		/// if there is no lock, then create it and return Some.
		fn try_lock(&self) -> Option<Self::LockType<'_>>;

		/// Destroy the blocking structure and remove
		/// the lock (usually always involves just a drop)
		fn unlock(&self, lock_type: Self::LockType<'_>);

		/// The actual structure that holds the synchronization
		/// and provides access to the data.
		type LockType<'a>: Deref<Target = Self::DerefLockType> + DerefMut where Self: 'a;

		/// The data type to modify, provided by the synchronization structure.
		type DerefLockType;

		/// Whether the current lock is active
		#[cfg_attr(docsrs, doc(cfg( feature = "pl" )))]
		#[cfg( all(
			feature = "pl",

			not(feature = "std"),
			not(feature = "async")
		) )]
		fn is_lock(&self) -> bool;
	}
}

/// Universal synchronization point structure,
/// combining various types of locks and working with them.
#[repr(transparent)]
pub struct SyncPoint<T> {
	/// Generalized structure for generating locks.
	mutex_builder: T,
}

impl<T> SyncPoint<T>
where
	T: SyncPointBeh,
{
	/// Structure creation.
	#[inline]
	pub const fn new(mutex_builder: T) -> Self {
		Self { mutex_builder }
	}

	cfg_not_async! {
		/// Create a new hold lock.
		#[inline]
		pub fn new_lock(&self) -> T::LockType<'_> {
			T::new_lock(&self.mutex_builder)
		}
	}

	cfg_async! {
		/// Create a new hold lock.
		#[inline]
		pub async fn new_lock(&self) -> T::LockType<'_> {
			T::new_lock(&self.mutex_builder).await
		}
	}

	/// If the lock exists and is not released, then return None,
	/// if there is no lock, then create it and return Some.
	#[inline]
	pub fn try_lock(&self) -> Option<T::LockType<'_>> {
		T::try_lock(&self.mutex_builder)
	}

	/// Destroy the blocking structure and remove the lock
	/// (usually always involves just a drop).
	#[inline]
	pub fn unlock(&self, lock: T::LockType<'_>) {
		T::unlock(&self.mutex_builder, lock)
	}

	/// Whether the current lock is active
	#[inline]
	#[cfg_attr(docsrs, doc(cfg(feature = "pl")))]
	#[cfg(all(feature = "pl", not(feature = "std"), not(feature = "async")))]
	pub fn is_lock(&self) -> bool {
		T::is_lock(&self.mutex_builder)
	}
}
