use types::ResponseParameters;

use ::serde_json::Error as SerdeErr;

#[derive(Debug, Fail)]
pub enum RawTelegramError {
	#[fail(display = "Detached Error: {}", _0)]
	DetachedError(String),

	#[fail(display = "Empty Body")]
	EmptyBody,

	#[fail(display = "Telegram Error: {} => {:?}", description, parameters)]
	TelegramError {
		description: String,
		parameters: Option<ResponseParameters>,
	},

	#[fail(display = "Json Error")]
	JsonError {
		#[fail(cause)]
		cause: SerdeErr
	}
}

impl From<SerdeErr> for RawTelegramError {
	fn from(e: SerdeErr) -> Self {
		RawTelegramError::JsonError {
			cause: e
		}
	}
}
