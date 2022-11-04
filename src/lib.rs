//Copyright 2022 #UlinProject Denis Kotlyarov (Денис Котляров)

//Licensed under the Apache License, Version 2.0 (the "License");
//you may not use this file except in compliance with the License.
//You may obtain a copy of the License at

//	   http://www.apache.org/licenses/LICENSE-2.0

//Unless required by applicable law or agreed to in writing, software
//distributed under the License is distributed on an "AS IS" BASIS,
//WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//See the License for the specific language governing permissions and
// limitations under the License.

// #Ulin Project 2022
//

/*!

Convenient and simple macro for code synchronization in multithreading.

# Use

### 1. easy/sync

```rust
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
```

### 2. sync_static

```rust
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
```

### 3. sync_let

```rust
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
			// Create anonymous synchronized code with one mutable variable `sync_let` and `count`.
			let result = synchronized!(
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

### 4. point

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

#[cfg( feature = "point" )]
use synchronized::synchronized_point;
#[cfg( feature = "point" )]
use synchronized::synchronized;

#[cfg( not(feature = "point") )]
macro_rules! synchronized_point {
	[ $($unk:tt)* ] => {
		println!("!!! This example requires support for the `point` feature. Run the example with `cargo run --example point --all-features`.");
	};
}

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
		let result1 = synchronized! ((->COMB_SYNC) {
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

# Connection

This section only describes how to choose the default synchronization method for a `synchronized` macro.

### 1. PlugAndPlay (minimal, sync, std)

For a `synchronized` macro, use the primitives implemented by the default `std` library.

```rust,ignore
[dependencies.synchronized]
version = "1.0.4"
default-features = false
features = [
	"std",
	#"point",
	#"get_point_name"
]
```

### 2. PlugAndPlay (minimal, sync, parking_lot)

For a `synchronized` macro, use the primitives implemented by the default `parking_lot` library.

```rust,ignore
[dependencies.synchronized]
version = "1.0.4"
default-features = false
features = [
	"parking_lot",
	#"point",
	#"get_point_name"
]
```

### 3. PlugAndPlay (minimal, async, tokio+parking_lot+async_trait)

For a `synchronized` macro, use the primitives implemented by the default `tokio` library.

```rust,ignore
[dependencies.synchronized]
version = "1.0.4"
default-features = false
features = [
	"async",
	#"point",
	#"get_point_name"
]
```

# Additionally inf

1. The macro is an alternative to the `synchronized` keyword from the Java programming language for the Rust programming language with all sorts of extensions.

2. This macro was created by an author who has not written in Java for a very long time, inspired by the memory of the Java programming language (versions 1.5-1.6).

*/

#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[macro_use]
mod async_core;
pub mod core;
#[cfg( feature = "point" )]
#[cfg_attr(docsrs, doc(cfg(feature = "point")))]
mod point;


mod macro_features;

/// Various synchronization primitives used in the `synchronized` macro.
pub mod beh {
	#[cfg_attr(docsrs, doc(cfg( feature = "parking_lot" )))]
	#[cfg( feature = "parking_lot" )]
	pub mod pl;
	
	#[cfg_attr(docsrs, doc(cfg( feature = "std" )))]
	#[cfg( feature = "std" )]
	pub mod std;
	
	#[cfg_attr(docsrs, doc(cfg( feature = "async" )))]
	#[cfg( feature = "async" )]
	//cfg_only_async! {
	pub mod r#async;
	//}
	
	// If locking is not selected, then select `std` by default.
	#[cfg(
		any(
			all(
				not(feature = "parking_lot"),
				not(feature = "std"),
				not(feature = "async")
			),
			/*all(
				feature = "parking_lot",
				feature = "std",
				feature = "async"
			)*/
		)
	)]
	pub mod std;
}

/// Convenient and simple macro for code synchronization in multithreading.
/// 
/// ### 1. Anonymous code is synchronized in multi-threaded mode.
/// ```rust
/// use synchronized::synchronized;
/// 
/// synchronized! {
///		static mut POINT0: usize = 0;
///		static mut POINT1: usize = 0;
///		
///		unsafe {
///			POINT1 = POINT0;
///			POINT0 += 1;
///			
///			POINT1
///		}
/// };
/// ```
/// 
/// ### 2. Create anonymous synchronized code with one variable to change `sync_let` and `count`.
/// ```rust
///	use synchronized::synchronized;
///	
///	let result = synchronized!((sync_let: String = String::new(), count: usize = 0) {
///		// We fill the variable `sync_let` with data.
///		sync_let.push_str("1 ");
///	
///		*count += 1;
///		sync_let.clone()
///	});
/// ```
#[macro_export]
macro_rules! synchronized {
	{
		// Named `$sync_point_name` synchronized block with mutable value 
		// of synchronized name `$v_point_name`, type `$ty` and value when 
		// `$expr` is created.
		// (Use only with `synchronized_point`.)
		->$sync_point_name:ident ( $($v_point_name: ident),* $(,)? ) $($all:tt)*
	} => {{ // synchronized point
		$crate::__synchronized_beh!(#new_lock(__lock): $sync_point_name);
		
		let ( $(ref mut $v_point_name),* ) = *__lock;
		let result = {
			$($all)*
		};
		$(
			drop($v_point_name);
		)*
		
		$crate::__synchronized_beh!(#drop_lock(__lock): $sync_point_name);
		
		result
	}};
	
	{
		// Named `$sync_point_name` synchronized block with mutable value 
		// of synchronized name `$v_point_name`, type `$ty` and value when 
		// `$expr` is created.
		$sync_point_name:ident ( $v_point_name: ident: $ty: ty = $expr:expr $(,)? ) $($all:tt)*
	} => {{
		$crate::__synchronized_beh!(#new_point<$ty: [$expr]>: $sync_point_name);
		$crate::synchronized! {
			->$sync_point_name ($v_point_name) $($all)*
		}
	}};
	
	{
		// Named sync block $sync_point_name with mutable values written 
		// comma-separated sync name $v_point_name, type $ty and value when 
		// $expr was created.
		$sync_point_name:ident ( $($v_point_name: ident: $ty: ty = $expr:expr),* $(,)? ) $($all:tt)*
	} => {{
		$crate::__synchronized_beh!(#new_point<($($ty),*): [($($expr),*)]>: $sync_point_name);
		$crate::synchronized! {
			->$sync_point_name ( $($v_point_name),* ) $($all)*
		}
	}};
	
	{
		// Named sync block named `$v_point_name`.
		// (Use only with `synchronized_point`.)
		(->$v_point_name: ident) $($all:tt)*
	} => {{ // sync point
		$crate::synchronized! {
			->$v_point_name (__empty_value) $($all)*
		}
	}};
	
	{
		// Named sync block named `$v_point_name`.
		($v_point_name: ident) $($all:tt)*
	} => {{
		$crate::synchronized! {
			$v_point_name (__empty_value: () = ()) $($all)*
		}
	}};
	{
		// Anonymous synchronized block with mutable synchronized name value 
		// `$v_point_name`, type `$ty` and value when `$expr` is created.
		( $($v_point_name: ident: $ty: ty = $expr:expr),* $(,)? ) $($all:tt)*
	} => {{ // sync value
		$crate::synchronized! {
			__ANONYMOUS_SYNC_POINT ( $($v_point_name: $ty = $expr),* ) $($all)*
		}
	}};
	
	{
		// COMPILE_ERROR
		$(->$_ident1:ident)? /* OR */ $($_ident2:ident)? ($($unk_in:tt)*) $($unk:tt)+
	} => {
		compile_error!(concat!(
			"Error writing macro `synchronized`, incode: ",
			$(stringify!(->$_ident1),)? 
			$(stringify!($_ident2),)? 
			
			stringify!(($($unk_in)*)),
			
			stringify!($($unk)+),
		));
	};
	
	{
		// Anonymous synchronized block
		$($all:tt)*
	} => {{ // nohead synchronized block
		$crate::synchronized! {
			(__empty_value: () = ()) $($all)*
		}
	}};
	
	[] => {}
}

/// Describes the selected default lock for the `synchronized` macro. Currently it is `
#[doc = crate::__synchronized_beh!( #name )]
/// `.
pub const CURRENT_DEF_BEH: &'static str = crate::__synchronized_beh!( #name );

/// Whether `get_point_name` was enabled in this build.
/// 
/// The `get_point_name` feature determines whether the connection 
/// label name can be determined at run time.
pub const IS_GET_POINT_NAME_SUPPORT: bool = {
	#[cfg( not(feature = "get_point_name") )] {
		false
	}
	
	#[cfg( feature = "get_point_name" )] {
		true
	}
};

/// Whether synchronized `point` was enabled in this build.
/// 
/// The `synchronized_point` macro allows you to create shared synchronization 
/// points that prevent certain code from being executed at the same time.
pub const IS_SYNC_POINT_SUPPORT: bool = {
	#[cfg( not(feature = "point") )] {
		false
	}
	
	#[cfg( feature = "point" )] {
		true
	}
};

cfg_only_async! {
	/// Determines if the library code and its locks are fully asynchronous.
	/// Currently it is `true`.
	pub const IS_ALWAYS_ASYNC: bool = true;
}

cfg_not_only_async! {
	/// Determines if the library code and its locks are fully asynchronous.
	/// Currently it is `false`.
	pub const IS_ALWAYS_ASYNC: bool = false;
}
