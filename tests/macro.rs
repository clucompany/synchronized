
use synchronized::synchronized;
#[cfg( all( feature = "parking_lot", not( feature = "std" ) ) )]
use synchronized::synchronized_point;

#[cfg(test)]
#[test]
fn test_synchronized() {
	synchronized! {
		let a = 1 + 2;
		
		assert_eq!(a, 3);
	}
	
	let result = synchronized! {
		let a = 1 + 2;
		
		assert_eq!(a, 3);
		
		a
	};
	assert_eq!(result, 3);
	
	let result = synchronized!((test: String = String::new()) {
		assert_eq!(test.is_empty(), true);
		
		*test = "test".to_string();
		test.clone()
	});
	assert_eq!(result, "test");
}

#[cfg(test)]
#[cfg( all( feature = "parking_lot", not( feature = "std" ) ) )]
#[test]
fn test_synchronized_point() {
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

