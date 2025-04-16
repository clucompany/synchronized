//! Synchronization primitive for the `synchronized`
//! macro implemented by the `tokio`+`parking_lot` library.

extern crate tokio;

use crate::core::SyncPointBeh;
use crate::core::r#async::cfg_async_or_sync;
pub use tokio::sync::Mutex;
pub use tokio::sync::MutexGuard;

cfg_async_or_sync! {
	impl[T: Send] SyncPointBeh for Mutex<T> {
		/// This section of code is connected only if
		/// the current library is asynchronous.
		#only_async {
			#[inline]
			async fn new_lock(&self) -> Self::LockType<'_> {
				Mutex::lock(self).await
			}

			#[inline]
			async fn try_lock(&self) -> Option<Self::LockType<'_>> {
				Mutex::try_lock(self).ok()
			}

			#[inline]
			async fn unlock(&self, lock_type: Self::LockType<'_>) {
				drop(lock_type)
			}
		}
		/// This section of code is connected only if
		/// the current library is synchronous.
		#only_sync {
			#[inline]
			fn new_lock<'a>(&'a self) -> Self::LockType<'a> {
				unimplemented!();
			}

			#[inline]
			fn try_lock<'a>(&'a self) -> Option<Self::LockType<'a>> {
				unimplemented!();
			}

			#[inline]
			fn unlock<'a>(&'a self, _lock_type: Self::LockType<'a>) {
				unimplemented!();
			}
		}

		type LockType<'a> = MutexGuard<'a, T> where T: 'a;
		type DerefLockType = T;
	}
}

/// An implementation specifying which synchronization to use in synchonized.
///
/// Required for implementation:
///
/// 1. #new_point<$t: ty : [$t_make:expr]>: $v_point_name:ident
/// Defining a new synchronization point, usually implements static variables used during synchronization.
/// 2. #new_lock($lock:ident): $v_point_name:ident
/// Creates a new lock on an already created sync point (#new_point)
/// 3. #drop_lock($lock: ident): $v_point_name:ident
/// Deletes a newly created lock (#new_lock)
/// 4. #name
/// Definition of the current implementation
#[macro_export]
#[doc(hidden)]
#[cfg(not(any(feature = "parking_lot", feature = "std")))]
macro_rules! __sync_beh {
	{
		// Definition of the current implementation
		#name
	} => { "async(tokio+parking_lot+async_trait)" };

	{
		// Defining a new synchronization point, usually implements static
		// variables used during synchronization.
		#new_point<$t: ty : [$t_make:expr]>: $v_point_name:ident
	} => {

		/// Generated Synchronization Point
		#[allow(dead_code)]
		#[allow(non_upper_case_globals)]
		#[allow(non_camel_case_types)]
		pub static $v_point_name: $crate::core::SyncPoint<
			$crate::beh::r#async::Mutex<$t>
		> = $crate::core::SyncPoint::new($crate::beh::r#async::Mutex::const_new(
			$t_make
		));
	};
	{
		// Creates a new lock on an already created sync point (#new_point)
		#new_lock($lock:ident): $v_point_name:ident
	} => {
		#[allow(unused_mut)]
		let mut $lock = $v_point_name.new_lock().await;
	};
	{
		// Deletes a newly created lock (#new_lock)
		#drop_lock($lock: ident): $v_point_name:ident
	} => {
		$crate::core::SyncPoint::unlock(&$v_point_name, $lock).await;
	};
}
