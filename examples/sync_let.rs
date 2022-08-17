
use std::thread::spawn;
use synchronized::synchronized;

/*
	An example that describes how to quickly create an anonymous 
	sync with a mutable variable.
	
	This code creates 5 threads, each of which tries to update 
	the `sync_let` variable with data while executing the synchronized anonymous code.
*/

fn main() {
	// An array of handles to wait for all threads to complete.
	let mut join_all = Vec::new();
	
	// Creation of 5 threads to implement a multi-threaded environment.
	for thread_id in 0..5 {
		let join = spawn(move || {
			// Create anonymous synchronized code with one mutable variable `sync_let`.
			let result = synchronized!((sync_let: String = String::new()) {
				// If it's the first thread, 
				// then theoretically `sync_let` is String::new().
				if thread_id == 0 {
					assert_eq!(sync_let.is_empty(), true);
				}
				
				// We fill the variable `sync_let` with data.
				sync_let.push_str(&thread_id.to_string());
				sync_let.push_str(" ");
				
				sync_let.clone()
			});
			
			// Outputting debug information.
			println!("#[id: {}] {}", thread_id, result);
		});
		
		// In order for our `assert_eq!(sync_let.is_empty());` code to 
		// always run correctly, the first thread should always run first 
		// (this is just for the stability of this example).
		if thread_id == 0 {
			let _e = join.join();
			continue;
		}
		
		join_all.push(join);
	}
	
	// We just wait for all threads to finish and look at stdout.
	for tjoin in join_all {
		let _e = tjoin.join();
	}
}
