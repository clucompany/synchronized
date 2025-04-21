/*
	Quick implementation examples of blocking anonymous code.
*/

#[cfg(not(feature = "async"))]
fn main() {
	use synchronized::sync;

	// #1 Anonymous inter-threaded synchronized code,
	// in the case of multi-threading, one thread will wait for the completion of another.
	sync! {
		println!("1");
	}

	// #2 Anonymous inter-threaded synchronized code,
	// in the case of multi-threading, one thread will wait for the completion of another.
	sync!( println!("1"); );
}

#[cfg(feature = "async")]
fn main() {
	println!("This example only builds and runs with --feature=\"sync\"");
}
