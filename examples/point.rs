
use synchronized::synchronized_point;
use synchronized::synchronized;

/*
	An example implementation of synchronized code with 
	one non-anonymous synchronization point.
	
	This example creates a set of anonymous sync codes associated with a 
	single named sync point. Each synchronization code executes in the same 
	way as ordinary anonymous code, but execution occurs simultaneously in a 
	multi-threaded environment in only one of them.
*/

fn main() {
	// A sync point named `COMB_SYNC` to group anonymous code syncs by name.
	synchronized_point! {(COMB_SYNC) {
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
		
		// This line of code is not synchronized and can run concurrently on all threads.
		println!("Unsynchronized code");
		
		// #2 Anonymous synchronized code that operates on a 
		// single named synchronization point.
		//
		// Note that `result0` and `result1` cannot be calculated at the same time, 
		// this does not happen because `result0` or `result1` are calculated in 
		// synchronized code with a single sync point of the same name.
		let result1 = synchronized! ((->COMB_SYNC) {
			println!("SyncCode, name_point: {}", COMB_SYNC.get_sync_point_name());
			unsafe {
				POINT+= 1;
				
				POINT
			}
		});
		
		// Display debug information.
		println!("result, res0: {:?}, res1: {:?}", result0, result1);
	}}
}