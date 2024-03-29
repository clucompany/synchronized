
//! This build.rs doesn't actually do anything other than display a warning if 
//! `features` were not installed correctly.

#[cfg( 
	all(
		not(test), 
		not(docsrs),
		
		not(feature = "doc_cfg"),
		feature = "parking_lot",
		feature = "std",
		feature = "async",
	)
)]
#[inline(always)]
fn main() {
	/// CargoWarningPrintln
	macro_rules! cwarning {
		[ @const: $a: expr  ] => {
			println!( concat!("cargo:warning=", $a) );
		};
		[ $a: expr] => {
			println!( "cargo:warning={}", $a );
		};
		[ $a: expr, $a2: expr] => {
			println!( "cargo:warning={}: {}", $a, $a2 );
		};
	}
	
	cwarning!(@const: "synchronized: The choice of synchronization between `std` and `parking_lot`, and async(tokio+parking_lot) was expected by default for `synchronized` macro. It is not possible to use both synchronization methods at the same time, `std` is now used.");
}

#[cfg( 
	not(all(
		not(test), 
		not(docsrs),
		
		not(feature = "doc_cfg"),
		feature = "parking_lot",
		feature = "std",
		feature = "async",
	))
)]
#[inline(always)]
fn main() {}
