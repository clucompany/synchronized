//! Various synchronization primitives used in the `synchronized` macro.

#[cfg_attr(
	docsrs,
	doc(cfg(any(
		feature = "std",
		all(not(feature = "pl"), not(feature = "std"), not(feature = "async"))
	)))
)]
#[cfg(any(
	feature = "std",
	all(not(feature = "pl"), not(feature = "std"), not(feature = "async"))
))]
pub mod std;

#[cfg_attr(docsrs, doc(cfg(feature = "pl")))]
#[cfg(feature = "pl")]
pub mod pl;

#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
#[cfg(feature = "async")]
//cfg_async! {
pub mod r#async;
//}
