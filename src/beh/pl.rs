
extern crate parking_lot;

use crate::core::BehSyncPoint;
pub use parking_lot::Mutex;
pub use parking_lot::MutexGuard;
pub use parking_lot::const_mutex;
pub use parking_lot::Once;

impl<'a, T> BehSyncPoint for &'a Mutex<T> {
	// !!! ATTENTION
	// Due to the inability to make <'a> in stable growth, 
	// you have to do &'a and then make strange types out of it.
	//
	type LockType = MutexGuard<'a, T>;
	
	#[inline(always)]
	fn lock(&self) -> Self::LockType {
		Mutex::lock(self)
	}
	
	#[inline(always)]
	fn is_lock(&self) -> bool {
		Mutex::is_locked(self)
	}
	
	#[inline(always)]
	fn try_lock(&self) -> Option<Self::LockType> {
		Mutex::try_lock(self)
	}
	
	#[inline(always)]
	fn unlock(&self, lock_type: Self::LockType) {
		drop(lock_type)
	}
}

#[doc(hidden)]
#[macro_export]
macro_rules! __sync_point {
	{ @point: $name_point:ident } => {
		#[allow(dead_code)]
		static CONST_MUTEX: $crate::beh::pl::Mutex<()> = $crate::beh::pl::const_mutex(());
		
		#[allow(dead_code)]
		pub static $name_point: $crate::core::SyncPoint<
			&'static $crate::beh::pl::Mutex<()>
		> = $crate::core::SyncPoint::const_new(&CONST_MUTEX);
	};
	{ @lock: $name_point:ident, $lock:ident } => {
		#[allow(unused_mut)]
		let mut $lock = $name_point.lock();
	};
	{ @drop: $name_point:ident, $lock: ident } => {
		$crate::core::SyncPoint::unlock(&$name_point, $lock);
	};
}

