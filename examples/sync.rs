
use synchronized::synchronized;

/*
	Quick implementation examples of blocking anonymous code.
*/

fn main() {
	// #1 Anonymous inter-threaded synchronized code, 
	// in the case of multi-threading, one thread will wait for the completion of another.
	synchronized! {
		println!("1");
	}
	
	// #2 Anonymous inter-threaded synchronized code, 
	// in the case of multi-threading, one thread will wait for the completion of another.
	synchronized!( println!("1"); );
}
