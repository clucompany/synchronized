#[cfg(all(test, not(feature = "async")))]
mod test_noasync {
	use synchronized::sync;

	#[test]
	fn test_synchronized() {
		sync! {
			let a = 1 + 2;

			assert_eq!(a, 3);
		}

		let result = sync! {
			let a = 1 + 2;

			assert_eq!(a, 3);

			a
		};
		assert_eq!(result, 3);

		let result = sync!((test: String = String::new()) {
			assert!(test.is_empty());

			*test = "test".to_string();
			test.clone()
		});
		assert_eq!(result, "test");
	}
}

#[cfg(all(test, feature = "pl", feature = "point", not(feature = "async")))]
mod test_noasync_onlypoints {
	use synchronized::sync;

	use synchronized::sync_point;

	#[test]
	fn test_sync_point() {
		sync_point! ((NAME_SYNC_POINT) {
			sync!((NAME_SYNC_POINT) {
				assert!(NAME_SYNC_POINT.is_lock());
			});

			// unsync block
			// ..
			assert_eq!(NAME_SYNC_POINT.is_lock(), false);

			sync!((NAME_SYNC_POINT) {
				assert!(NAME_SYNC_POINT.is_lock());
			});
		});
	}
}
