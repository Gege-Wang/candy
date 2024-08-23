use super::Task;
use alloc::collections::VecDeque;
use core::task::{Context, Poll};
use core::task::{RawWaker, RawWakerVTable, Waker};
pub struct TaskQueue {
    task_queue: VecDeque<Task>,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            task_queue: VecDeque::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        self.task_queue.push_back(task);
    }

    pub fn run(&mut self) {
        while let Some(mut task) = self.task_queue.pop_front() {
            let waker = dummy_waker();
            let mut context = Context::from_waker(&waker);
            match task.poll(&mut context) {
                Poll::Pending => self.task_queue.push_back(task),
                Poll::Ready(()) => {},
            }
        }
    }
}

fn dummy_raw_weaker() -> RawWaker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        dummy_raw_weaker()
    }

    let vtable = &RawWakerVTable::new(clone, no_op, no_op, no_op);
    RawWaker::new(core::ptr::null(), vtable)
}

fn dummy_waker() -> Waker {
    unsafe {
        Waker::from_raw(dummy_raw_weaker())
    }
}