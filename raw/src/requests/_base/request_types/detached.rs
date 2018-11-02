use requests::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize)]
pub struct DetachedRequestType;

impl RequestType for DetachedRequestType {
	type Options = ();
	type Request = Result<HttpRequest, RawTelegramError>;

	fn serialize(_options: Self::Options, request: &Self::Request) -> Result<HttpRequest, RawTelegramError> {
		match request {
			&Ok(ref req) => Ok(req.clone()),
			&Err(ref err) => Err(RawTelegramError::DetachedError(err.to_string())),
		}
	}
}
