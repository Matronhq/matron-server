#![cfg(test)]
#![allow(unused_features)] // 1.96.0-nightly 2026-03-07 bug

use insta::{assert_debug_snapshot, with_settings};
use matron_server::{Args, Runtime, Server};
use matron_server_core::Result;

#[test]
fn admin_execute_echo() -> Result {
	with_settings!({
		description => "Admin Execute Echo",
		snapshot_suffix => "admin_execute_echo",
	}, {
		let mut args = Args::default_test(&["smoke", "fresh", "cleanup"]);
		args.execute.push("debug echo Test".into());

		let runtime = Runtime::new(Some(&args))?;
		let server = Server::new(Some(&args), Some(&runtime))?;
		let result = runtime.block_on(async {
			matron_server::async_exec(&server).await
		});

		drop(runtime);
		assert_debug_snapshot!(result);
		result
	})
}
