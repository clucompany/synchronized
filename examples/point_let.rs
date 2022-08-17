
use synchronized::synchronized_point;
use synchronized::synchronized;

/*
	An example of the implementation of synchronized code with one non-anonymous (named) 
	synchronization point with one mutable variable.
*/

fn main() {
	// A sync point named `COMB_SYNC` to group anonymous code syncs by name.
	//
	// Note that the sync point has a mutable String variable with a default value 
	// of String::new(). To make this variable mutable, you will need to call the sync macro.
	synchronized_point! {COMB_SYNC (String = String::new()) {
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
				POINT+= 1;
				
				POINT
			}
		});
		
		// #1 This line of code is not synchronized and can run concurrently on all threads.
		println!("Unsynchronized code 1");
		
		// Synchronized code by `COMB_SYNC` label with `sync_let: String` mutable variable
		let result1 = synchronized!(->COMB_SYNC(sync_let) {
			// sync_let <-- String (COMB_SYNC)
			*sync_let = "test".to_string();
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
				POINT+= 1;
				
				POINT
			}
		});
		
		// Display debug information.
		println!("result, res0: {:?}, res1: {:?}, res2: {:?}", result0, result1, result2);
	}}
}
