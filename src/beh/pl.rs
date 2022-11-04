
//! Synchronization primitive for the `synchronized` 
//! macro implemented by the `parking_lot` library.

extern crate parking_lot;

use crate::core::SyncPointBeh;
pub use parking_lot::Mutex;
pub use parking_lot::MutexGuard;
pub use parking_lot::const_mutex;

impl<T> SyncPointBeh for Mutex<T> {
	type LockType<'a> = MutexGuard<'a, T> where T: 'a;
	type DerefLockType = T;
	
	#[inline(always)]
	fn new_lock<'a>(&'a self) -> Self::LockType<'a> {
		Mutex::lock(self)
	}
	
	#[inline(always)]
	#[cfg_attr(docsrs, doc(cfg( feature = "parking_lot" )))]
	#[cfg( all(
		feature = "parking_lot",
		
		not(feature = "std"),
		not(feature = "async")
	) )]
	fn is_lock(&self) -> bool {
		Mutex::is_locked(self)
	}
	
	#[inline(always)]
	fn try_lock<'a>(&'a self) -> Option<Self::LockType<'a>> {
		Mutex::try_lock(self)
	}
	
	#[inline(always)]
	fn unlock<'a>(&'a self, lock_type: Self::LockType<'a>) {
		drop(lock_type)
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
/// 
#[doc(hidden)]
#[cfg( all(not(any( feature = "std", feature = "async" ))) )]
#[macro_export]
macro_rules! __synchronized_beh {
	{
		// Definition of the current implementation
		#name
	} => { "parking_lot" };

	{
		// Defining a new synchronization point, usually implements static 
		// variables used during synchronization.
		#new_point<$t: ty : [$t_make:expr]>: $v_point_name:ident
	} => {
		$crate::__make_name!( #new_name<_HIDDEN_NAME>: stringify!($v_point_name) );
		
		/// Generated Synchronization Point
		#[allow(dead_code)]
		#[allow(non_upper_case_globals)]
		pub static $v_point_name: $crate::core::SyncPoint<
			$crate::beh::pl::Mutex<$t>,
			$crate::__make_name!( #get_name<_HIDDEN_NAME> )
		> = $crate::core::SyncPoint::new($crate::beh::pl::const_mutex(
			$t_make
		));
	};
	{
		// Creates a new lock on an already created sync point (#new_point)
		#new_lock($lock:ident): $v_point_name:ident
	} => {
		#[allow(unused_mut)]
		let mut $lock = $v_point_name.new_lock();
	};
	{
		// Deletes a newly created lock (#new_lock)
		#drop_lock($lock: ident): $v_point_name:ident
	} => {
		$crate::core::SyncPoint::unlock(&$v_point_name, $lock);
	};
}

