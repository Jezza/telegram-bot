use url::TELEGRAM_URL;

//use http::{Method as HMethod, Request as HReq, Response as HRes};
//pub type HttpRequest0 = HReq<Vec<u8>>;
//		let t = ReqB::new()
//			.method(Method::GET)
//			.body(body);
//		let t = Req::post(url)
//			.body(body)
//			.map_err(RawTelegramError::from);

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HttpRequest {
	url: &'static str,
	body: Vec<u8>,
}

impl HttpRequest {
	pub fn new(url: &'static str, body: Vec<u8>) -> Self {

//		let t = Req::post(url)
//			.body(body)
//			.map_err(RawTelegramError::from);

		HttpRequest {
			url,
			body,
		}
	}

	pub fn into_body(self) -> Vec<u8> {
		self.body
	}

	pub fn body(&self) -> &Vec<u8> {
		&self.body
	}

	pub fn url(&self, token: &str) -> String {
		format!("{}bot{}/{}", TELEGRAM_URL, token, self.url)
	}
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HttpResponse {
	pub body: Option<Vec<u8>>
}

