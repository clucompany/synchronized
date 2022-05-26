
#![no_std]

pub mod core;
pub mod beh {
	pub mod pl;
}

#[macro_export]
macro_rules! synchronized {
	( ($name_point: ident) $($all:tt)*) => {{
		$crate::__sync_point!(@lock: $name_point, __lock);
		{
			$($all)*
		}
		
		$crate::__sync_point!(@drop: $name_point, __lock);
	}};
	{ $($all:tt)* } => {{
		$crate::__sync_point!(@point: __PL_SYNC_POINT);
		$crate::synchronized!((__PL_SYNC_POINT) $($all)*);
	}};
}

#[macro_export]
macro_rules! synchronized_point {
	( $( ($name_point:ident) {$($all:tt)*} );+ $(;)? ) => {
		$({
			$crate::__sync_point!(@point: $name_point);
			
			$($all)*
		})+
	};
}

#[cfg(test)]
#[test]
fn test_macros() {
	synchronized! {
		let a = 1 + 2;
		
		assert_eq!(a, 3);
	}
	
	synchronized_point! ((NAME_SYNC_POINT) {
		synchronized!((NAME_SYNC_POINT) {
			assert_eq!(NAME_SYNC_POINT.is_lock(), true);
			
		});
		
		// unsync block
		// ..
		assert_eq!(NAME_SYNC_POINT.is_lock(), false);
		
		synchronized!((NAME_SYNC_POINT) {
			assert_eq!(NAME_SYNC_POINT.is_lock(), true);
			
		});
	});
}
