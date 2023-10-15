//! Defines a set of adapters for converting between channel types at a type-level.
//! In particular, these are useful when some memory may contain elements of different types
//! And so channels of different types may be connected to the memory.

use crate::{context_tools::DAMType, structures::TimeManager};

use super::{ChannelElement, DequeueError, EnqueueError, PeekResult, Receiver, Sender};

/// An adapter for Receivers, delegating and converting all underlying operations
pub trait RecvAdapter<U> {
    /// See: [Receiver::peek]
    fn peek(&self) -> PeekResult<U>;
    /// See: [Receiver::peek_next]
    fn peek_next(&self, manager: &TimeManager) -> Result<ChannelElement<U>, DequeueError>;
    /// See: [Receiver::dequeue]
    fn dequeue(&self, manager: &TimeManager) -> Result<ChannelElement<U>, DequeueError>;
}

impl<T: DAMType, U> RecvAdapter<U> for Receiver<T>
where
    T: Into<U>,
{
    fn peek(&self) -> PeekResult<U> {
        match Receiver::peek(self) {
            PeekResult::Something(ce) => PeekResult::Something(ce.convert()),
            PeekResult::Nothing(time) => PeekResult::Nothing(time),
            PeekResult::Closed => PeekResult::Closed,
        }
    }

    fn peek_next(&self, manager: &TimeManager) -> Result<ChannelElement<U>, DequeueError> {
        Receiver::peek_next(self, manager).map(|val| val.convert())
    }

    fn dequeue(&self, manager: &TimeManager) -> Result<ChannelElement<U>, DequeueError> {
        Receiver::dequeue(self, manager).map(|val| val.convert())
    }
}

/// An adapter for Senders, delegating and converting all underlying operations.
pub trait SendAdapter<U> {
    /// See: [Sender::enqueue]
    fn enqueue(&self, manager: &TimeManager, data: ChannelElement<U>) -> Result<(), EnqueueError>;

    /// See: [Sender::wait_until_available]
    fn wait_until_available(&self, manager: &mut TimeManager) -> Result<(), EnqueueError>;
}

impl<T: DAMType, U> SendAdapter<U> for Sender<T>
where
    T: From<U>,
{
    fn enqueue(&self, manager: &TimeManager, data: ChannelElement<U>) -> Result<(), EnqueueError> {
        Sender::enqueue(&self, manager, data.convert())
    }

    fn wait_until_available(&self, manager: &mut TimeManager) -> Result<(), EnqueueError> {
        Sender::wait_until_available(self, manager)
    }
}
