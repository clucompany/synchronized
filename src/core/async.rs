use crate::cfg::{cfg_async, cfg_not_async};

cfg_async! {
	/// A macro that determines whether to add asynchronous fn support for traits.
	macro_rules! cfg_async_or_sync {
		[
			$(#[$($addmeta:tt)*])*
			pub trait $name_trait: ident {
				$(#[$doc_hide0:meta])* // doc hidden
				#only_async {
					$($async_code:tt)*
				}
				$(#[$doc_hide1:meta])* // doc hidden
				#only_sync	{
					$($sync_code:tt)*
				}

				$($code:tt)+
			}
		] => {
			$(#[$($addmeta)*])*
			pub trait $name_trait {
				$($async_code)* // << (A<<<)SYNC_CODE

				$($code)+
			}
		};
		[
			$(#[$($addmeta:tt)*])*
			impl $([$($left:tt)*])? $name_trait: ident for $impl_ty: ty {
				$(#[$doc_hide0:meta])* // doc hidden
				#only_async	{
					$($async_code:tt)*
				}
				$(#[$doc_hide1:meta])* // doc hidden
				#only_sync	{
					$($sync_code:tt)*
				}

				$($code:tt)+
			}
		] => {
			$(#[$($addmeta)*])*
			impl $(<$($left)*>)? $name_trait for $impl_ty {
				$($async_code)* // << (A<<<)SYNC_CODE

				$($code)+
			}
		};
	}
}

cfg_not_async! {
	/// A macro that determines whether to add asynchronous fn support for traits.
	macro_rules! cfg_async_or_sync {
		[
			$(#[$($addmeta:tt)*])*
			pub trait $name_trait: ident {
				$(#[$doc_hide0:meta])* // doc hidden
				#only_async	{
					$($async_code:tt)*
				}
				$(#[$doc_hide1:meta])* // doc hidden
				#only_sync	{
					$($sync_code:tt)*
				}

				$($code:tt)+
			}
		] => {
			$(#[$($addmeta)*])*
			pub trait $name_trait {
				$($sync_code)* // << SYNC_CODE

				$($code)+
			}
		};
		[
			$(#[$($addmeta:tt)*])*
			impl $([$($left:tt)*])? $name_trait: ident for $impl_ty: ty {
				$(#[$doc_hide0:meta])* // doc hidden
				#only_async	{
					$($async_code:tt)*
				}
				$(#[$doc_hide1:meta])* // doc hidden
				#only_sync	{
					$($sync_code:tt)*
				}

				$($code:tt)+
			}
		] => {
			$(#[$($addmeta)*])*
			impl $(<$($left)*>)? $name_trait for $impl_ty {
				$($sync_code)* // << SYNC_CODE

				$($code)+
			}
		};
	}
}

pub(crate) use cfg_async_or_sync;
