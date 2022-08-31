
//! Description of the optional `synchronized_point` macro.

/// Create a named code synchronization point.
/// It is required in order to combine two or more synchronized places in the code, 
/// excluding their simultaneous execution.
/// 
/// ### 1. One named sync point and two or more sync codes for it.
/// ```rust
///	use synchronized::synchronized_point;
///	use synchronized::synchronized;
///	
///	synchronized_point! {(COMB_SYNC) {
///		static mut POINT: usize = 0;
///		
///		// #1 Anonymous synchronized code that operates on a single named synchronization point.
///		let result0 = synchronized! ((->COMB_SYNC) {
///			println!("SyncCode, name_point: {}", COMB_SYNC.get_sync_point_name());
///			unsafe {
///				POINT += 1;
///				
///				POINT
///			}
///		});
///	
///		// This line of code is not synchronized and can run concurrently on all threads.
///		println!("Unsynchronized code");
///	
///		// #2 Anonymous synchronized code that operates on a single named synchronization point.
///		let result1 = synchronized! ((->COMB_SYNC) {
///			println!("SyncCode, name_point: {}", COMB_SYNC.get_sync_point_name());
///			unsafe {
///				POINT += 1;
///			
///				POINT
///			}
///		});
///	}}
/// ```
/// 
/// ### 2. One named sync point and two or more sync codes for it. With one mutable variable.
/// ```rust
///	use synchronized::synchronized;
///	use synchronized::synchronized_point;
///	
///	synchronized_point! {COMB_SYNC (String = String::new()) {
///		static mut POINT: usize = 0;
///		
///		// #1 Anonymous synchronized code that operates on a single named synchronization point.
///		let result0 = synchronized! ((->COMB_SYNC) {
///			println!("SyncCode, name_point: {}", COMB_SYNC.get_sync_point_name());
///			unsafe {
///				POINT += 1;
///				
///				POINT
///			}
///		});
///	
///		// #1 This line of code is not synchronized and can run concurrently on all threads.
///		println!("Unsynchronized code");
///		
///		// Synchronized code by `COMB_SYNC` label with `sync_let: String` mutable variable
///		let result1 = synchronized!(->COMB_SYNC(sync_let) {
///			// sync_let <-- String (COMB_SYNC)
///			*sync_let = "test".to_string();
///		});
///		
///		// #2 This line of code is not synchronized and can run concurrently on all threads.
///		println!("Unsynchronized code");
///		
///		// #2 Anonymous synchronized code that operates on a single named synchronization point.
///		let result2 = synchronized! ((->COMB_SYNC) {
///			println!("SyncCode, name_point: {}", COMB_SYNC.get_sync_point_name());
///			unsafe {
///				POINT += 1;
///			
///				POINT
///			}
///		});
///	}}
#[macro_export]
#[cfg_attr(docsrs, doc(cfg(feature = "point")))]
macro_rules! synchronized_point {
	{
		// Named sync point named `$sync_point_name`.
		//
		// With a mutable synchronized variable of type `$ty` 
		// with a default value of `$expr`.
		$sync_point_name:ident ( $ty: ty = $expr:expr $(,)? ) {$($all:tt)*} $(; $($unk:tt)*)?
	} => {
		{
			$crate::__synchronized_beh!(#new_point<$ty: [$expr]>: $sync_point_name);
			
			$($all)*
		}
		
		$($crate::synchronized_point! {
			$($unk)*
		})?
	};
	{
		// Named sync point named `$sync_point_name`.
		//
		// With mutable synchronized comma-separated variables of type `$ty`
		// with a default value of `$expr`.
		$sync_point_name:ident ( $($ty: ty = $expr:expr),* $(,)? ) {$($all:tt)*} $(; $($unk:tt)*)?
	} => {
		{
			$crate::__synchronized_beh!(#new_point<($($ty),*): [($($expr),*)]>: $sync_point_name);
			
			$($all)*
		}
		
		$($crate::synchronized_point! {
			$($unk)*
		})?
	};
	{ 	
		// Named sync point named `$sync_point_name`
		($sync_point_name:ident) {$($all:tt)*} $(; $($unk:tt)*)? 
	} => {
		$crate::synchronized_point! {
			$sync_point_name (() = ()) { $($all)* }
			
			$(; $($unk)*)?
		}
	};
	
	{ 
		// COMPILE_ERROR
		$($unk:tt)+
	} => {
		compile_error!(concat!(
			"Error writing macro `synchronized_point`, incode: ",
			stringify!($($unk)+),
		));
	};
	
	[] => {}
}
