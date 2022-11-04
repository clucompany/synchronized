
use crate::{cfg_only_async, cfg_not_only_async};

cfg_only_async! {
	#[doc(hidden)]
	#[macro_export]
	/// A macro that determines whether to add asynchronous fn support for traits.
	macro_rules! async_or_sync_code {
		[
			$(#[$($addmeta:tt)*])*
			pub trait $name_trait: ident {
				$(#[$doc_hide0:meta])* // doc hidden
				#only_async	{ $($async_code:tt)* }
				$(#[$doc_hide1:meta])* // doc hidden
				#only_sync	{ $($sync_code:tt)* }
				
				$($code:tt)+
			}
		] => {
			extern crate alloc;
			use alloc::boxed::Box;
			use async_trait::async_trait;
			
			$(#[$($addmeta)*])*
			#[async_trait]
			pub trait $name_trait {
				$($async_code)* // << (A<<<)SYNC_CODE
				
				$($code)+
			}
		};
		[
			$(#[$($addmeta:tt)*])*
			impl $([$($left:tt)*])? $name_trait: ident for $impl_ty: ty {
				$(#[$doc_hide0:meta])* // doc hidden
				#only_async	{ $($async_code:tt)* }
				$(#[$doc_hide1:meta])* // doc hidden
				#only_sync	{ $($sync_code:tt)* }
				
				$($code:tt)+
			}
		] => {
			extern crate alloc;
			use alloc::boxed::Box;
			use async_trait::async_trait;
			
			$(#[$($addmeta)*])*
			#[async_trait]
			impl $(<$($left)*>)? $name_trait for $impl_ty {
				$($async_code)* // << (A<<<)SYNC_CODE
				
				$($code)+
			}
		};
	}
}

cfg_not_only_async! {
	#[doc(hidden)]
	#[macro_export]
	/// A macro that determines whether to add asynchronous fn support for traits.
	macro_rules! async_or_sync_code {
		[
			$(#[$($addmeta:tt)*])*
			pub trait $name_trait: ident {
				$(#[$doc_hide0:meta])* // doc hidden
				#only_async	{ $($async_code:tt)* }
				$(#[$doc_hide1:meta])* // doc hidden
				#only_sync	{ $($sync_code:tt)* }
				
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
				#only_async	{ $($async_code:tt)* }
				$(#[$doc_hide1:meta])* // doc hidden
				#only_sync	{ $($sync_code:tt)* }
				
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
