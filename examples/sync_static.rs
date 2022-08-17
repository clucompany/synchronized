
use std::thread::spawn;
use synchronized::synchronized;

/*
	A more illustrative example of code blocking implementation 
	for SAFE mutability of two or more static variables.
	
	The code creates 5 threads that try to change static variables at the same 
	time without synchronization, but the code synchronization block 
	does not allow this code to execute at the same time, which makes 
	the implementation of changing static variables SAFE and even somewhat 
	easier and more beneficial in terms of use and performance.
*/

fn main() {
	// An array of handles to wait for all threads to complete.
	let mut join_all = Vec::new();
	
	// Creation of 5 threads to implement a multi-threaded environment.
	for thread_id in 0..5 {
		let join = spawn(move || {
			// Print the thread number and print the 
			// result of the sync_fn function.
			println!("#[id: {}] {}", thread_id, sync_fn());
		});
		
		join_all.push(join);
	}
	
	// We just wait for all threads to finish and look at stdout.
	for tjoin in join_all {
		let _e = tjoin.join();
	}
}

fn sync_fn() -> usize {
	// Create anonymous synchronized code.
	//
	// The code will never run at the same time. If one thread is executing 
	// this code, the second thread will wait for this code to finish executing.
	let result = synchronized! {
		static mut POINT0: usize = 0;
		static mut POINT1: usize = 0;
		
		unsafe {
			POINT1 = POINT0;
			POINT0 += 1;
			
			POINT1
		}
	};
	
	result
}
