#[cfg(feature = "response-channel")]
use crossbeam_channel::Receiver;

use crate::errors::Result;
#[cfg(feature = "response-channel")]
use crate::response::Response;
use crate::Event;

/// `Sender` is responsible for handling events after Send() is called.  Implementations
/// of `send()` must be safe for concurrent calls.
pub trait Sender {
    /// `send` queues up an event to be sent
    fn send(&mut self, ev: Event);

    /// `start` initializes any background processes necessary to send events
    fn start(&mut self);

    /// `stop` flushes any pending queues and blocks until everything in flight has been
    /// sent
    fn stop(&mut self) -> Result<()>;

    /// `responses` returns a channel that will contain a single Response for each Event
    /// added. Note that they may not be in the same order as they came in
    #[cfg(feature = "response-channel")]
    fn responses(&self) -> Receiver<Response>;
}
