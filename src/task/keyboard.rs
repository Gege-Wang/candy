use crossbeam_queue::ArrayQueue;
use conquer_once::spin::OnceCell;
use futures_util::stream::Stream;
use futures_util::task::Context;
use futures_util::task::Poll;
use core::pin::Pin;
use futures_util::task::AtomicWaker;

use crate::println;
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();
pub(crate) fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            println!("Scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        println!("Scancode queue uninitialized; dropping keyboard input");
    }

}

pub struct ScancodeStream {
    _private: (),
}

impl ScancodeStream {
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(|| {
            ArrayQueue::new(100)
        }).expect("Scancode queue already initialized");
        ScancodeStream { _private: () }
    }
}

impl Stream for ScancodeStream {
    type Item = u8;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE.try_get().expect("not initialized");
        if let Some(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }
        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))

            },
            None => Poll::Pending,
        }
    }
}
