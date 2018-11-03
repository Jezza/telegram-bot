//! Connector with tokio-curl backend.

use curl::easy::{Easy, List};
use errors::TelegramError;
use future::TelegramFuture;
use futures::Future;
use futures::future::result;
use parking_lot::Mutex;
use std::fmt;
use std::rc::Rc;
use std::sync::Arc;
use telegram_bot_raw::{HttpRequest, HttpResponse};
use tokio_core::reactor::Handle;
use tokio_curl::Session;

pub type Connector = CurlConnector;

/// This connector uses `tokio-curl` backend.
pub struct CurlConnector {
	inner: Rc<Session>
}

impl fmt::Debug for CurlConnector {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("CurlConnector")
	}
}

impl CurlConnector {
	fn new(handle: &Handle) -> Self {
		CurlConnector {
			inner: Rc::new(Session::new(handle.clone()))
		}
	}

	pub fn request(&self, token: &str, req: HttpRequest) -> impl TelegramFuture<HttpResponse> {
		let session = self.inner.clone();

		result(self.create_request(token, req))
			.and_then(move |(handle, result)| {
				session.perform(handle)
					   .map_err(From::from)
					   .join(Ok(result))
			})
			.and_then(move |(_, result)| {
				let mut swap = Vec::new();
				let mut guard = result.lock();
				let prev: &mut Vec<u8> = &mut guard;
				::std::mem::swap(prev, &mut swap);
				Ok(HttpResponse {
					body: Some(swap)
				})
			})
	}

	fn create_request(&self, token: &str, request: HttpRequest) -> Result<(Easy, Arc<Mutex<Vec<u8>>>), TelegramError> {
		let mut handle = Easy::new();

		let url = request.url(token);
		handle.url(&url)?;

//		handle.get(true)?;

		handle.post(true)?;
		handle.post_fields_copy(&request.into_body())?;

		let mut headers = List::new();
		headers.append("Content-Type: application/json")?;
		handle.http_headers(headers)?;

		let result = Arc::new(Mutex::new(Vec::new()));
		let write_result = result.clone();

		handle.write_function(move |data| {
			write_result.lock().extend_from_slice(data);
			Ok(data.len())
		})?;

		Ok((handle, result))
	}
}

/// Returns default curl connector.
pub fn create_connector(handle: &Handle) -> Result<CurlConnector, TelegramError> {
	Ok(CurlConnector::new(handle))
}
