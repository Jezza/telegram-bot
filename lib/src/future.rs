use futures::{Future, Poll};
use errors::TelegramError;

/// Represent a future that resolves into Telegram API response.
#[must_use = "futures do nothing unless polled"]
pub struct TelegramFuture<T> {
	inner: Box<Future<Item=T, Error=TelegramError>>,
}

pub trait NewTelegramFuture<T> {
	fn new(inner: Box<Future<Item=T, Error=TelegramError>>) -> Self;
}

impl<T> NewTelegramFuture<T> for TelegramFuture<T> {
	fn new(inner: Box<Future<Item=T, Error=TelegramError>>) -> Self {
		Self { inner: inner }
	}
}

impl<T> Future for TelegramFuture<T> {
	type Item = T;
	type Error = TelegramError;

	fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
		self.inner.poll()
	}
}
