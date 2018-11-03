//! This crate helps writing bots for the messenger Telegram.
//! See [readme](https://github.com/telegram-rs/telegram-bot) for details.

extern crate telegram_bot_raw;

extern crate tokio_core;

extern crate parking_lot;

#[macro_use]
extern crate failure;

extern crate futures;

#[cfg(feature = "hyper_connector")]
extern crate hyper;
#[cfg(feature = "hyper_connector")]
extern crate hyper_tls;

#[cfg(feature = "curl_connector")]
extern crate curl;
#[cfg(feature = "curl_connector")]
extern crate tokio_curl;

pub use connector::*;
pub use prelude::*;
pub use self::api::Api;
pub use self::errors::TelegramError;
pub use self::future::TelegramFuture;
pub use stream::UpdatesStream;
pub use types::*;

mod api;
mod errors;
mod future;
mod macros;
mod stream;

pub mod connector;
pub mod prelude;
pub mod types;

