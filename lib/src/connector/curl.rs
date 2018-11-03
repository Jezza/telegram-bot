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
use telegram_bot_raw::{Body, HttpRequest, HttpResponse, Method};
use tokio_core::reactor::Handle;
use tokio_curl::Session;

/// This connector uses `tokio-curl` backend.
pub struct Connector {
	inner: Rc<Session>
}

impl fmt::Debug for Connector {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.write_str("CurlConnector")
	}
}

impl Connector {
	fn new(handle: &Handle) -> Self {
		Connector {
			inner: Rc::new(Session::new(handle.clone()))
		}
	}

	pub fn request(&self, token: &str, req: HttpRequest) -> impl TelegramFuture<HttpResponse> {
		let request = result(self.create_request(token, req));

		let session = self.inner.clone();
		let request = request.and_then(move |(handle, result)| {
			session.perform(handle).map_err(From::from).join(Ok(result))
		});

		request.and_then(move |(_, result)| {
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

		let url = request.url.url(token);
		handle.url(&url)?;

		match request.method {
			Method::Get => handle.get(true)?,
			Method::Post => handle.post(true)?,
		}

		match request.body {
			Body::Empty => (),
			Body::Json(body) => {
				handle.post_fields_copy(&body)?;

				let mut headers = List::new();
				headers.append(&format!("Content-Type: application/json"))?;
				handle.http_headers(headers)?;
			}
		}

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
pub fn default_connector(handle: &Handle) -> Result<Connector, TelegramError> {
	Ok(Connector::new(handle))
}
