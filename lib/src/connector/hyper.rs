//! Connector with hyper backend.

use errors::TelegramError;
use future::TelegramFuture;
use futures::{Future, Stream};
use futures::future::result;
use hyper::{Body, Request, Uri};
use hyper::client::{Client, HttpConnector};
use hyper::client::connect::dns::GaiResolver;
use hyper::header;
use hyper_tls::HttpsConnector;
use std::rc::Rc;
use std::str::FromStr;
use telegram_bot_raw::{HttpRequest, HttpResponse};
use tokio_core::reactor::Handle;

type HClient = Client<HttpsConnector<HttpConnector<GaiResolver>>>;

/// This connector uses `hyper` backend.
pub struct Connector {
	inner: Rc<HClient>
}

impl Connector {
	fn new(client: HClient) -> Self {
		Connector {
			inner: Rc::new(client)
		}
	}

	pub fn request(&self, token: &str, req: HttpRequest) -> impl TelegramFuture<HttpResponse> {
		let client = self.inner.clone();
		result(Uri::from_str(&req.url(token)))
			.map_err(From::from)
			.and_then(move |uri| {
//				client.get(uri)
				client.request(Request::post(uri)
					.header(header::CONTENT_TYPE, "application/json")
					.body(Body::from(req.into_body()))
					.unwrap())
					  .map_err(From::from)
			})
			.and_then(|response| {
				response.into_body()
						.concat2()
						.map_err(From::from)
			})
			.and_then(|body| {
				Ok(HttpResponse {
					body: Some(body.to_vec())
				})
			})
	}
}

/// Returns a hyper connector. Uses one resolve thread and `HttpsConnector`.
pub fn create_connector(_handle: &Handle) -> Result<Connector, TelegramError> {
	let connector = HttpsConnector::new(1)
		.map_err(TelegramError::from)?;
	let client: HClient = Client::builder()
		.build(connector);
	Ok(Connector::new(client))
}
