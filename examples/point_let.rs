
#[cfg( feature = "point" )]
use synchronized::synchronized_point;
#[cfg( feature = "point" )]
use synchronized::synchronized;


/*
	An example of the implementation of synchronized code with one non-anonymous (named) 
	synchronization point with one mutable variable.
	
	!!! In this example, the assembly requires the `point` feature to be active.
*/

#[cfg( not(feature = "point") )]
macro_rules! synchronized_point {
	[ $($unk:tt)* ] => {
		println!("!!! This example requires support for the `point` feature. Run the example with `cargo run --example point_let --all-features`.");
	};
}

fn main() {
	// A sync point named `COMB_SYNC` to group anonymous code syncs by name.
	//
	// Note that the sync point has a mutable `String` and `usize` variable with a default value 
	// of `String::new()` and `0`. To make this variable mutable, you will need to call the sync macro.
	synchronized_point! {COMB_SYNC (String = String::new(), usize = 0) {
		static mut POINT: usize = 0;
		println!("GeneralSyncPoint, name_point: {}", COMB_SYNC.get_sync_point_name());
		
		// #1 Anonymous synchronized code that operates on a 
		// single named synchronization point.
		//
		// This code is not executed concurrently in a multi-threaded environment, 
		// one thread is waiting for someone else's code to execute in this part of the code.
		let result0 = synchronized! ((->COMB_SYNC) {
			println!("SyncCode, name_point: {}", COMB_SYNC.get_sync_point_name());
			unsafe {
				POINT += 1;
				
				POINT
			}
		});
		
		// #1 This line of code is not synchronized and can run concurrently on all threads.
		println!("Unsynchronized code 1");
		
		// Synchronized code by `COMB_SYNC` label with `sync_let: String` and `count: usize` mutable variable
		let result1 = synchronized!(->COMB_SYNC(sync_let, count) {
			// sync_let <-- String (COMB_SYNC)
			*sync_let = "test".to_string();
			*count += 1;
			
			*count
		});
		
		// #2 This line of code is not synchronized and can run concurrently on all threads.
		println!("Unsynchronized code 2");
		
		// #2 Anonymous synchronized code that operates on a 
		// single named synchronization point.
		//
		// Note that `result0` and `result1` cannot be calculated at the same time, 
		// this does not happen because `result0` or `result1` or `result2` are calculated in 
		// synchronized code with a single sync point of the same name.
		let result2 = synchronized! ((->COMB_SYNC) {
			println!("SyncCode, name_point: {}", COMB_SYNC.get_sync_point_name());
			unsafe {
				POINT += 1;
				
				POINT
			}
		});
		
		// Display debug information.
		println!("result, res0: {:?}, res1: {:?}, res2: {:?}", result0, result1, result2);
	}}
}
