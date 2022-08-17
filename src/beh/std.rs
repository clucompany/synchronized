
//! Synchronization primitive for the `synchronized` 
//! macro implemented by the `std` library.

extern crate std;

use std::sync::MutexGuard;
use crate::core::SyncPointBeh;
pub use std::sync::Mutex;

impl<'a, T> SyncPointBeh for &'a Mutex<T> {
	// !!! ATTENTION
	// Due to the inability to make <'a> in stable growth, 
	// you have to do &'a and then make strange types out of it.
	//
	type LockType = MutexGuard<'a, T>;
	type DerefLockType = T;
	
	#[inline(always)]
	fn new_lock(&self) -> Self::LockType {
		match Mutex::lock(self) {
			Ok(a) => a,
			Err(e) => e.into_inner(),
		}
	}
	
	#[cfg_attr(docsrs, doc(cfg(feature = "parking_lot")))]
	#[cfg( feature = "parking_lot" )]
	#[inline(always)]
	fn is_lock(&self) -> bool {
		true
	}
	
	#[inline(always)]
	fn try_lock(&self) -> Option<Self::LockType> {
		match Mutex::try_lock(self) {
			Ok(a) => Some(a),
			_ => None,
		}
	}
	
	#[inline(always)]
	fn unlock(&self, lock_type: Self::LockType) {
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
#[macro_export]
macro_rules! __synchronized_beh {
	// Definition of the current implementation
	{ #name } => { "std" };
	
	// Defining a new synchronization point, usually implements static variables used during synchronization.
	{ #new_point<$t: ty : [$t_make:expr]>: $v_point_name:ident } => {
		$crate::__make_name!( #new_name<_HIDDEN_NAME>: stringify!($v_point_name) );
		
		#[allow(dead_code)]
		static CONST_MUTEX: $crate::beh::std::Mutex<$t> = $crate::beh::std::Mutex::new(
			$t_make
		);
		
		/// Generated Synchronization Point
		#[allow(dead_code)]
		#[allow(non_upper_case_globals)]
		pub static $v_point_name: $crate::core::SyncPoint<
			&'static $crate::beh::std::Mutex<$t>, 
			$crate::__make_name!(#get_name<_HIDDEN_NAME>)
		> = $crate::core::SyncPoint::new(&CONST_MUTEX);
	};
	// Creates a new lock on an already created sync point (#new_point)
	{ #new_lock($lock:ident): $v_point_name:ident } => {
		#[allow(unused_mut)]
		let mut $lock = $v_point_name.new_lock();
	};
	// Deletes a newly created lock (#new_lock)
	{ #drop_lock($lock: ident): $v_point_name:ident } => {
		$crate::core::SyncPoint::unlock(&$v_point_name, $lock);
	};
}
