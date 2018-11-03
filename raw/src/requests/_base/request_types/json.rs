use requests::*;
use serde::Serialize;
use serde_json;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct JsonRequestType<Request> {
	phantom: ::std::marker::PhantomData<Request>,
}

impl<R: Serialize> RequestType for JsonRequestType<R> {
	type Options = &'static str;
	type Request = R;

	fn serialize(url: Self::Options, request: &Self::Request) -> Result<HttpRequest, RawTelegramError> {
		let body = serde_json::to_vec(&request)?;
		Ok(HttpRequest::new(url, body))
	}
}
