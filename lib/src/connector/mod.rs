//! IO backend.
//!
//! `CurlConnector` is default connector unless feature `curl_connector` is disabled and
//! feature `hyper_connector` is enabled. This behaviour will change after hyper release.

pub use self::_base::Connector;
#[cfg(feature = "curl_connector")]
pub use self::curl::CurlConnector;
#[cfg(feature = "hyper_connector")]
pub use self::hyper::HyperConnector;
pub use super::errors::TelegramError;
use tokio_core::reactor::Handle;

mod _base;
#[cfg(feature = "curl_connector")]
pub mod curl;
#[cfg(feature = "hyper_connector")]
pub mod hyper;

/// Returns default connector.
///
/// See module level documentation for details.
#[cfg(feature = "curl_connector")]
pub fn default_connector(handle: &Handle) -> Result<Box<Connector>, TelegramError> {
	curl::default_connector(handle)
}

/// Returns default connector.
///
/// See module level documentation for details.
#[cfg(all(not(feature = "curl_connector"), all(feature = "hyper_connector")))]
pub fn default_connector(handle: &Handle) -> Result<Box<Connector>, TelegramError> {
	hyper::default_connector(handle)
}