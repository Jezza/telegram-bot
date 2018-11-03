use errors::TelegramError;
use futures::Future;

// Represent a future that resolves into Telegram API response.
#[must_use = "futures do nothing unless polled"]
pub trait TelegramFuture<T>: Future<Item=T, Error=TelegramError> {}

impl<I, T> TelegramFuture<I> for T
	where T: Future<Item=I, Error=TelegramError> {}
