
// # cfg_async

#[doc(hidden)]
#[macro_export]
#[cfg(all(
	feature = "async",
	
	not(feature = "parking_lot"),
	not(feature = "std"),
))]
macro_rules! cfg_async {
	[ $($code:tt)+ ] => {
		#[cfg_attr(docsrs, doc(cfg( feature = "async" )))]
		$($code)+
	}
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(all(
	feature = "async",
	
	not(feature = "parking_lot"),
	not(feature = "std"),
)))]
macro_rules! cfg_async {
	[ $($code:tt)+ ] => {}
}

// # cfg_not_async

#[doc(hidden)]
#[macro_export]
#[cfg(all(
	feature = "async",
	
	not(feature = "parking_lot"),
	not(feature = "std"),
))]
macro_rules! cfg_not_async {
	[ $($code:tt)+ ] => {
		
	}
}

#[doc(hidden)]
#[macro_export]
#[cfg(not(all(
	feature = "async",
	
	not(feature = "parking_lot"),
	not(feature = "std"),
)))]
macro_rules! cfg_not_async {
	[ $($code:tt)+ ] => {
		#[cfg_attr(docsrs, doc(cfg( not(feature = "async") )))]
		$($code)+
	}
}
