
//! Synchronization primitive for the `synchronized` 
//! macro implemented by the `tokio`+`parking_lot` library.

extern crate tokio;

pub use tokio::sync::Mutex;
pub use tokio::sync::MutexGuard;
use crate::core::SyncPointBeh;
use crate::cfg_not_async;
use crate::cfg_async;

cfg_async! {
	macro_rules! async_or_sync_impltraitcode {
		[
			$(#[$($addmeta:tt)*])*
			impl[$($left:tt)*] $name_trait: ident for $impl_ty: ty {
				#async	{ $($async_code:tt)* }
				#sync	{ $($sync_code:tt)* }
				
				$($code:tt)+
			}
		] => {
			extern crate alloc;
			use alloc::boxed::Box;
			use async_trait::async_trait;
			
			$(#[$($addmeta)*])*
			#[async_trait]
			impl<$($left)*> $name_trait for $impl_ty {
				$($async_code)*
				
				$($code)+
			}
		};
	}
}
cfg_not_async! {
	macro_rules! async_or_sync_impltraitcode {
		[
			$(#[$($addmeta:tt)*])*
			impl[$($left:tt)*] $name_trait: ident for $impl_ty: ty {
				#async	{ $($async_code:tt)* }
				#sync	{ $($sync_code:tt)* }
				
				$($code:tt)+
			}
		] => {
			$(#[$($addmeta)*])*
			impl<$($left)*> $name_trait for $impl_ty {
				$($sync_code)*
				
				$($code)+
			}
		};
	}
}

async_or_sync_impltraitcode! {
	impl['a, T: Send] SyncPointBeh for &'a Mutex<T> {
		#async {
			#[inline(always)]
			async fn new_lock(&self) -> Self::LockType {
				Mutex::lock(self).await
			}
			
			#[inline(always)]
			async fn try_lock(&self) -> Option<Self::LockType> {
				match Mutex::try_lock(self) {
					Ok(a) => Some(a),
					_ => None,
				}
			}
			
			#[inline(always)]
			async fn unlock(&self, lock_type: Self::LockType) {
				drop(lock_type)
			}
		}
		#sync {
			#[inline(always)]
			fn new_lock(&self) -> Self::LockType {
				unimplemented!();
			}
			
			#[inline(always)]
			fn try_lock(&self) -> Option<Self::LockType> {
				unimplemented!();
			}
			
			#[inline(always)]
			fn unlock(&self, _lock_type: Self::LockType) {
				unimplemented!();
			}
		}
		
		// !!! ATTENTION
		// Due to the inability to make <'a> in stable growth, 
		// you have to do &'a and then make strange types out of it.
		//
		type LockType = MutexGuard<'a, T>;
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
/// 
#[doc(hidden)]
#[cfg( all(not(any( feature = "parking_lot", feature = "std" ))) )]
#[macro_export]
macro_rules! __synchronized_beh {
	{
		// Definition of the current implementation
		#name
	} => { "async(tokio+parking_lot)" };

	{
		// Defining a new synchronization point, usually implements static 
		// variables used during synchronization.
		#new_point<$t: ty : [$t_make:expr]>: $v_point_name:ident
	} => {
		$crate::__make_name!( #new_name<_HIDDEN_NAME>: stringify!($v_point_name) );
		
		#[allow(dead_code)]
		static CONST_MUTEX: $crate::beh::r#async::Mutex<$t> = $crate::beh::r#async::Mutex::const_new(
			$t_make
		);
		
		/// Generated Synchronization Point
		#[allow(dead_code)]
		#[allow(non_upper_case_globals)]
		pub static $v_point_name: $crate::core::SyncPoint<
			&'static $crate::beh::r#async::Mutex<$t>, 
			$crate::__make_name!( #get_name<_HIDDEN_NAME> )
		> = $crate::core::SyncPoint::new(&CONST_MUTEX);
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
