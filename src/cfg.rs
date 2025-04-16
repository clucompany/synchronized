//! Additional macros for code generation depending on the `features` used.

// # cfg_async

/// Code is passed from the macro only if the `async` function is clearly defined.
#[cfg(all(feature = "async", not(feature = "parking_lot"), not(feature = "std"),))]
macro_rules! cfg_async {
	[ $($code:tt)+ ] => {
		#[cfg_attr(docsrs, doc(cfg( feature = "async" )))]
		$($code)+
	}
}

/// Code is passed from the macro only if the `async` function is clearly defined.
#[cfg(not(all(feature = "async", not(feature = "parking_lot"), not(feature = "std"),)))]
macro_rules! cfg_async {
	[ $($code:tt)+ ] => {}
}

pub(crate) use cfg_async;

// # cfg_not_async

/// Code is not passed from a macro only if the asynchronous function is clearly defined.
#[cfg(all(feature = "async", not(feature = "parking_lot"), not(feature = "std"),))]
macro_rules! cfg_not_async {
	[ $($code:tt)+ ] => {}
}

/// Code is not passed from a macro only if the asynchronous function is clearly defined.
#[cfg(not(all(feature = "async", not(feature = "parking_lot"), not(feature = "std"),)))]
macro_rules! cfg_not_async {
	[ $($code:tt)+ ] => {
		#[cfg_attr(docsrs, doc(cfg( not(feature = "async") )))]
		$($code)+
	}
}

pub(crate) use cfg_not_async;
