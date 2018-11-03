
use std::io::Error as IOError;

use telegram_bot_raw::RawTelegramError;

#[cfg(feature = "curl_connector")]
use curl::Error as CurlError;
#[cfg(feature = "curl_connector")]
use tokio_curl::PerformError as TokioCurlError;

#[cfg(feature = "hyper_connector")]
use hyper::Error as HyperError;
#[cfg(feature = "hyper_connector")]
use hyper::http::Error as HyperHttpError;
#[cfg(feature = "hyper_connector")]
use hyper::http::uri::InvalidUri as HyperUriError;
#[cfg(feature = "hyper_connector")]
use hyper_tls::Error as HyperTlsError;

#[derive(Debug, Fail)]
pub enum TelegramError {
	#[fail(display = "__unreachable__")]
	#[doc(hidden)]
	__Unreachable,

	#[fail(display = "IO Error")]
	IO {
		#[fail(cause)]
		cause: IOError,
	},

	#[cfg(feature = "hyper_connector")]
	#[fail(display = "Hyper Error")]
	Hyper {
		#[fail(cause)]
		cause: HyperError,
	},

	#[cfg(feature = "hyper_connector")]
	#[fail(display = "Hyper URI Error")]
	HyperUri {
		#[fail(cause)]
		cause: HyperUriError,
	},

	#[cfg(feature = "hyper_connector")]
	#[fail(display = "Hyper Http Error")]
	HyperHttp {
		#[fail(cause)]
		cause: HyperHttpError
	},

	#[cfg(feature = "hyper_connector")]
	#[fail(display = "Hyper TLS Error")]
	HyperTls {
		#[fail(cause)]
		cause: HyperTlsError
	},

	#[cfg(feature = "curl_connector")]
	#[fail(display = "Curl Error")]
	Curl {
		#[fail(cause)]
		cause: CurlError
	},

	#[fail(display = "Raw Error")]
	Raw {
		#[fail(cause)]
		cause: RawTelegramError
	},
}

// Raw(telegram_bot_raw::Error, telegram_bot_raw::ErrorKind);

impl From<IOError> for TelegramError {
	fn from(e: IOError) -> Self {
		TelegramError::IO {
			cause: e
		}
	}
}

#[cfg(feature = "hyper_connector")]
impl From<HyperError> for TelegramError {
	fn from(e: HyperError) -> Self {
		TelegramError::Hyper {
			cause: e
		}
	}
}

#[cfg(feature = "hyper_connector")]
impl From<HyperUriError> for TelegramError {
	fn from(e: HyperUriError) -> Self {
		TelegramError::HyperUri {
			cause: e
		}
	}
}

#[cfg(feature = "hyper_connector")]
impl From<HyperHttpError> for TelegramError {
	fn from(e: HyperHttpError) -> Self {
		TelegramError::HyperHttp {
			cause: e
		}
	}
}

#[cfg(feature = "hyper_connector")]
impl From<HyperTlsError> for TelegramError {
	fn from(e: HyperTlsError) -> Self {
		TelegramError::HyperTls {
			cause: e
		}
	}
}

#[cfg(feature = "curl_connector")]
impl From<CurlError> for TelegramError {
	fn from(e: CurlError) -> Self {
		TelegramError::Curl {
			cause: e
		}
	}
}

#[cfg(feature = "curl_connector")]
impl From<TokioCurlError> for TelegramError {
	fn from(e: TokioCurlError) -> Self {
		TelegramError::IO {
			cause: e.into_error()
		}
	}
}

impl From<RawTelegramError> for TelegramError {
	fn from(e: RawTelegramError) -> Self {
		TelegramError::Raw {
			cause: e
		}
	}
}
