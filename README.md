<div id="header" align="center">

  <b>[synchronized]</b>
  
  (Simple and convenient macro for synchronizing code in multithreading. )
  </br></br>

<div id="badges">
  <a href="./LICENSE_APACHE">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/apache2.png?raw=true" alt="apache2"/>
  </a>
  <a href="https://crates.io/crates/synchronized">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/cratesio.png?raw=true" alt="cratesio"/>
  </a>
  <a href="https://docs.rs/synchronized">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/docrs.png?raw=true" alt="docrs"/>
  </a>
  <a href="https://github.com/denisandroid">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/uproject.png?raw=true" alt="uproject"/>
  </a>
  <a href="https://github.com/clucompany">
    <img src="https://github.com/UlinProject/img/blob/main/short_32/clulab.png?raw=true" alt="clulab"/>
  </a>
	
  [![CI](https://github.com/clucompany/synchronized/actions/workflows/CI.yml/badge.svg?event=push)](https://github.com/clucompany/synchronized/actions/workflows/CI.yml) 


</div>
</div>


## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
synchronized = "1.1.0"
```

and this to your source code:

```rust
use synchronized::sync;
```

## Example

### 1. sync_static

```rust
use std::thread::spawn;
use synchronized::sync;

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
	let result = sync! {
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
```

### 2. point

```rust
/*
	An example implementation of synchronized code with 
	one non-anonymous synchronization point.
	
	This example creates a set of anonymous sync codes associated with a 
	single named sync point. Each synchronization code executes in the same 
	way as ordinary anonymous code, but execution occurs simultaneously in a 
	multi-threaded environment in only one of them.
	
	!!! In this example, the assembly requires the `point` feature to be active.
*/

use synchronized::sync_point;
use synchronized::sync;

fn main() {
	// A sync point named `COMB_SYNC` to group anonymous code syncs by name.
	sync_point! {(COMB_SYNC) {
		static mut POINT: usize = 0;
		println!("GeneralSyncPoint, name_point: {}", COMB_SYNC.get_sync_point_name());
		
		// #1 Anonymous synchronized code that operates on a 
		// single named synchronization point.
		//
		// This code is not executed concurrently in a multi-threaded environment, 
		// one thread is waiting for someone else's code to execute in this part of the code.
		let result0 = sync! ((->COMB_SYNC) {
			println!("SyncCode, name_point: {}", COMB_SYNC.get_sync_point_name());
			unsafe {
				POINT += 1;
				
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
		let result1 = sync! ((->COMB_SYNC) {
			println!("SyncCode, name_point: {}", COMB_SYNC.get_sync_point_name());
			unsafe {
				POINT += 1;
				
				POINT
			}
		});
		
		// Display debug information.
		println!("result, res0: {:?}, res1: {:?}", result0, result1);
	}}
}
```

### 3. sync_let

```rust
use std::thread::spawn;
use synchronized::sync;

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
			// Create anonymous synchronized code with one mutable variable `sync_let` and `count`.
			let result = sync!(
				(sync_let: String = String::new(), count: usize = 0) {
					// If it's the first thread, 
					// then theoretically `sync_let` is String::new().
					if thread_id == 0 {
						assert_eq!(sync_let.is_empty(), true);
						assert_eq!(count, &0);
					}
					
					// We fill the variable `sync_let` and `count` with data.
					sync_let.push_str(&thread_id.to_string());
					sync_let.push_str(" ");
					
					*count += 1;
					
					sync_let.clone()
				}
			);
			
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
```

<a href="./examples">
  See all
</a>


## Features

Synchronized supports locks from the standard `std` package, as well as the `parking_lot` package, and also supports asynchronous operation using locks from `tokio`:

### 1. `std` (only synchronization locks from the `std` library)

```rust,ignore
[dependencies.synchronized]
version = "1.1.0"
default-features = false
features = [
	"std",
	#"point", # Allows the use of synchronization points to avoid executing code in two or more places at the same time.
]
```

### 2. `parking_lot` (only synchronization locks from the `parking_lot` library)

```rust,ignore
[dependencies.synchronized]
version = "1.1.0"
default-features = false
features = [
	"pl",
	#"point", # Allows the use of synchronization points to avoid executing code in two or more places at the same time.
]
```

### 3. `tokio` (only async locks from `tokio` library)

```rust,ignore
[dependencies.synchronized]
version = "1.1.0"
default-features = false
features = [
	"async",
	#"point", # Allows the use of synchronization points to avoid executing code in two or more places at the same time.
]
```

## License

This project is distributed under the license (LICENSE-APACHE-2-0).

<div align="left">
  <a href="https://github.com/denisandroid">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/uproject.png?raw=true" alt="uproject"/>
  </a>
  <b>&nbsp;Copyright (c) 2022-2025 #UlinProject</b>
	
  <b>&nbsp;(Denis Kotlyarov).</b>
  </br></br></br>
</div>

### Apache License

<div align="left">
  <a href="./LICENSE_APACHE">
    <img align="left" src="https://github.com/UlinProject/img/blob/main/block_220_100/apache2.png?raw=true" alt="apache2"/>
    
  </a>
  <b>&nbsp;Licensed under the Apache License, Version 2.0.</b>
  </br></br></br></br>
</div>
