#![cfg(test)]
#![allow(unused_features)] // 1.96.0-nightly 2026-03-07 bug

use criterion::{Criterion, criterion_group, criterion_main};
use tracing::Level;
use matron_server::{Args, Runtime, Server};
use matron_server_core::result::ErrLog;

criterion_group!(
	name = benches;
	config = Criterion::default().sample_size(10).nresamples(1);
	targets = dummy, smoke
);

criterion_main!(benches);

fn dummy(c: &mut Criterion) { c.bench_function("dummy", |c| c.iter(|| {})); }

fn smoke(c: &mut Criterion) {
	let args = Args::default_test(&["fresh", "cleanup"]);
	let runtime = Runtime::new(Some(&args)).unwrap();
	let server = Server::new(Some(&args), Some(&runtime)).unwrap();

	runtime
		.block_on(async {
			matron_server::async_start(&server).await?;
			let run = matron_server::async_run(&server);
			c.bench_function("smoke", |c| {
				c.iter(|| {});
			});

			server.server.shutdown().log_err(Level::WARN).ok();
			run.await?;
			matron_server::async_stop(&server).await
		})
		.unwrap();

	drop(runtime);
}
