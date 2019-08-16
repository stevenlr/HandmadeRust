use core::{cell::RefCell, marker::PhantomPinned};
use fnd::containers::Queue;

pub enum Event
{
    DestroyWindow,
}

pub struct EventQueue
{
    queue: RefCell<Queue<Event>>,

    // We add this so EventQueue is !Unpin because we need
    // the raw pointer to the queue in the events callback.
    _pin: PhantomPinned,
}

impl EventQueue
{
    pub fn new() -> Self
    {
        Self {
            queue: RefCell::new(Queue::new()),
            _pin:  PhantomPinned,
        }
    }

    #[inline]
    pub fn queue_event(&self, event: Event)
    {
        self.queue.borrow_mut().push(event);
    }

    #[inline]
    pub fn poll_event(&self) -> Option<Event>
    {
        self.queue.borrow_mut().pop()
    }
}
