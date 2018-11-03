use types::ResponseParameters;

use ::serde_json::Error as SerdeErr;

#[derive(Debug, Fail)]
pub enum RawTelegramError {
	#[fail(display = "Detached Error: {}", _0)]
	Detached(String),

	#[fail(display = "Empty Body")]
	EmptyBody,

	#[fail(display = "Telegram Error: {} => {:?}", description, parameters)]
	Telegram {
		description: String,
		parameters: Option<ResponseParameters>,
	},

	#[fail(display = "Json Error")]
	Json {
		#[fail(cause)]
		cause: SerdeErr
	},
}

impl From<SerdeErr> for RawTelegramError {
	fn from(e: SerdeErr) -> Self {
		RawTelegramError::Json {
			cause: e
		}
	}
}
