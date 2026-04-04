use ruma::api::federation::discovery::get_server_version;
use matron_server_core::Result;

use crate::Ruma;

/// # `GET /_matrix/federation/v1/version`
///
/// Get version information on this server.
pub(crate) async fn get_server_version_route(
	_body: Ruma<get_server_version::v1::Request>,
) -> Result<get_server_version::v1::Response> {
	Ok(get_server_version::v1::Response {
		server: Some(get_server_version::v1::Server {
			name: Some(matron_server_core::version::name().into()),
			version: Some(matron_server_core::version::version().into()),
			compiler: matron_server_core::info::rustc::version().map(Into::into),
			..Default::default()
		}),
	})
}
