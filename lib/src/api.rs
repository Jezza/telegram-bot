use connector::{Connector, create_connector};
use errors::TelegramError;
use future::TelegramFuture;
use futures::Future;
use futures::future::result;
use std::borrow::Borrow;
use std::rc::Rc;
use std::time::Duration;
use stream::{NewUpdatesStream, UpdatesStream};
use telegram_bot_raw::{Request, ResponseType};
use tokio_core::reactor::{Handle, Timeout};

/// Main type for sending requests to the Telegram bot API.
#[derive(Clone)]
pub struct Api {
	inner: Rc<ApiInner>,
}

struct ApiInner {
	token: String,
	connector: Connector,
	handle: Handle,
}

impl Api {
	/// Start construction of the `Api` instance.
	///
	/// # Example
	///
	/// ```rust
	/// # extern crate telegram_bot;
	/// # extern crate tokio_core;
	/// use telegram_bot::Api;
	/// use tokio_core::reactor::Core;
	///
	/// # fn main() {
	/// let core = Core::new().unwrap();
	/// # let telegram_token = "token";
	/// let api = Api::create(telegram_token, core.handle()).unwrap();
	/// # }
	/// ```
	pub fn create<T: AsRef<str>, H: Borrow<Handle>>(token: T, handle: H) -> Result<Api, TelegramError> {
		let handle = handle.borrow().clone();
		// Connector API
		let connector = create_connector(&handle)?;
		Ok(Api {
			inner: Rc::new(ApiInner {
				token: token.as_ref().to_string(),
				connector: connector,
				handle,
			}),
		})
	}

	/// Create a stream which produces updates from the Telegram server.
	///
	/// # Examples
	///
	/// ```rust
	/// # extern crate futures;
	/// # extern crate telegram_bot;
	/// # extern crate tokio_core;
	/// # use telegram_bot::Api;
	/// # use tokio_core::reactor::Core;
	/// # fn main() {
	/// # let core = Core::new().unwrap();
	/// # let api: Api = Api::configure("token").build(core.handle()).unwrap();
	/// use futures::Stream;
	///
	/// let future = api.stream().for_each(|update| {
	///     println!("{:?}", update);
	///     Ok(())
	/// });
	/// # }
	/// ```
	pub fn stream(&self) -> UpdatesStream {
		UpdatesStream::new(self.clone(), self.inner.handle.clone())
	}

	/// Send a request to the Telegram server and do not wait for a response.
	///
	/// # Examples
	///
	/// ```rust
	/// # extern crate futures;
	/// # extern crate telegram_bot;
	/// # extern crate tokio_core;
	/// # use futures::Future;
	/// # use telegram_bot::{Api, GetMe, ChatId};
	/// # use telegram_bot::prelude::*;
	/// # use tokio_core::reactor::Core;
	/// #
	/// # fn main() {
	/// # let core = Core::new().unwrap();
	/// # let telegram_token = "token";
	/// # let api = Api::configure(telegram_token).build(core.handle()).unwrap();
	/// # if false {
	/// let chat = ChatId::new(61031);
	/// api.spawn(chat.text("Message"))
	/// # }
	/// # }
	pub fn spawn<Req: Request + 'static>(&self, request: Req) {
		self.inner.handle.spawn(self.send(request).then(|_| Ok(())))
	}

	/// Send a request to the Telegram server and wait for a response, timing out after `duration`.
	/// Future will resolve to `None` if timeout fired.
	///
	/// # Examples
	///
	/// ```rust
	/// # extern crate futures;
	/// # extern crate telegram_bot;
	/// # extern crate tokio_core;
	/// # use futures::Future;
	/// # use telegram_bot::{Api, GetMe};
	/// # use tokio_core::reactor::Core;
	/// #
	/// # fn main() {
	/// # let core = Core::new().unwrap();
	/// # let telegram_token = "token";
	/// # let api = Api::configure(telegram_token).build(core.handle()).unwrap();
	/// # if false {
	/// use std::time::Duration;
	///
	/// let future = api.send_timeout(GetMe, Duration::from_secs(5));
	/// future.and_then(|me| Ok(assert!(me.is_some())));
	/// # }
	/// # }
	/// ```
	pub fn send_timeout<Req: Request>(
		&self, request: Req, duration: Duration)
		-> impl TelegramFuture<Option<<Req::Response as ResponseType>::Type>> {
		let timeout_future = result(Timeout::new(duration, &self.inner.handle))
			.flatten()
			.map_err(From::from)
			.map(|()| None);

		let send_future = self.send(request)
							  .map(|resp| Some(resp));

		timeout_future.select(send_future)
					  .map(|(item, _next)| item)
					  .map_err(|(item, _next)| item)
	}

	/// Send a request to the Telegram server and wait for a response.
	///
	/// # Examples
	///
	/// ```rust
	/// # extern crate futures;
	/// # extern crate telegram_bot;
	/// # extern crate tokio_core;
	/// # use futures::Future;
	/// # use telegram_bot::{Api, GetMe};
	/// # use tokio_core::reactor::Core;
	/// #
	/// # fn main() {
	/// # let core = Core::new().unwrap();
	/// # let telegram_token = "token";
	/// # let api = Api::configure(telegram_token).build(core.handle()).unwrap();
	/// # if false {
	/// let future = api.send(GetMe);
	/// future.and_then(|me| Ok(println!("{:?}", me)));
	/// # }
	/// # }
	/// ```
	pub fn send<Req: Request>(&self, request: Req) -> impl TelegramFuture<<Req::Response as ResponseType>::Type> {
		let api = self.clone();

		result(request.serialize()
					  .map_err(From::from))
			.and_then(move |request| {
				let ref token = api.inner.token;
				// Connector API
				api.inner.connector.request(token, request)
			})
			.and_then(|response| {
				Req::Response::deserialize(response)
					.map_err(From::from)
			})
	}
}
