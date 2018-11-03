//! IO backend.
//!
//! If the curl connector is enabled, the curl module is loaded, and used.
//! If the hyper connector is enabled, instead the hyper connector is loaded, and used.
//! 
//! If no connector is enabled, the default connector is enabled, which just panics the process. (I need to work out how to cause a nice compile-time error...)

#[cfg(feature = "curl_connector")]
pub mod curl;
#[cfg(feature = "curl_connector")]
pub use self::curl::*;

#[cfg(feature = "hyper_connector")]
pub mod hyper;
#[cfg(feature = "hyper_connector")]
pub use self::hyper::*;

#[cfg(not(any(feature = "curl_connector", feature = "hyper_connector")))]
mod defaults {
	use std::fmt::{Debug, Formatter, Result as FResult};
	use errors::TelegramError;
	use future::TelegramFuture;
	use futures::future::err;
	use telegram_bot_raw::{HttpRequest, HttpResponse};
	use tokio_core::reactor::Handle;

	pub struct Connector;

	impl Connector {
		pub fn request(&self, _token: &str, _req: HttpRequest) -> impl TelegramFuture<HttpResponse> {
			err(TelegramError::__Unreachable)
		}
	}

	pub fn create_connector(_handle: &Handle) -> Result<Connector, TelegramError> {
		panic!("No connector specified.")
	}
}

#[cfg(not(any(feature = "curl_connector", feature = "hyper_connector")))]
pub use self::defaults::*;