#![cfg(test)]
#![allow(unused_features)] // 1.96.0-nightly 2026-03-07 bug

use insta::{assert_debug_snapshot, with_settings};
use matron_server::{Args, Runtime, Server};
use matron_server_core::Result;

#[test]
fn dummy() {}

#[test]
#[should_panic = "dummy"]
fn panic_dummy() { panic!("dummy") }

#[test]
fn smoke() -> Result {
	with_settings!({
		description => "Smoke Test",
		snapshot_suffix => "smoke_test",
	}, {
		let args = Args::default_test(&["smoke", "fresh", "cleanup"]);
		let runtime = Runtime::new(Some(&args))?;
		let server = Server::new(Some(&args), Some(&runtime))?;
		let result = matron_server::exec(&server, runtime);

		assert_debug_snapshot!(result);
		result
	})
}
