#![allow(unused_features)] // 1.96.0-nightly 2026-03-07 bug

mod handle;
mod layers;
mod request;
mod router;
mod run;
mod serve;

use std::{panic::AssertUnwindSafe, pin::Pin, sync::Arc};

use futures::{Future, FutureExt, TryFutureExt};
use log as _;
use matron_server_core::{Error, Result, Server};
use matron_server_service::Services;

matron_server_core::mod_ctor! {}
matron_server_core::mod_dtor! {}
matron_server_core::rustc_flags_capture! {}

#[unsafe(no_mangle)]
pub extern "Rust" fn start(
	server: &Arc<Server>,
) -> Pin<Box<dyn Future<Output = Result<Arc<Services>>> + Send>> {
	AssertUnwindSafe(run::start(server.clone()))
		.catch_unwind()
		.map_err(Error::from_panic)
		.unwrap_or_else(Err)
		.boxed()
}

#[unsafe(no_mangle)]
pub extern "Rust" fn stop(
	services: Arc<Services>,
) -> Pin<Box<dyn Future<Output = Result> + Send>> {
	AssertUnwindSafe(run::stop(services))
		.catch_unwind()
		.map_err(Error::from_panic)
		.unwrap_or_else(Err)
		.boxed()
}

#[unsafe(no_mangle)]
pub extern "Rust" fn run(
	services: &Arc<Services>,
) -> Pin<Box<dyn Future<Output = Result> + Send>> {
	AssertUnwindSafe(run::run(services.clone()))
		.catch_unwind()
		.map_err(Error::from_panic)
		.unwrap_or_else(Err)
		.boxed()
}
