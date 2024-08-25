use super::{Task, TaskId};
use alloc::collections::btree_map::BTreeMap;
use x86_64::instructions::interrupts;
use core::task::{Context, Poll};
use core::task::Waker;
use crossbeam_queue::ArrayQueue;
use alloc::sync::Arc;
use alloc::task::Wake;
pub struct TaskQueue {
    tasks: BTreeMap<TaskId, Task>,
    task_queue: Arc<ArrayQueue<TaskId>>,
    waker_cache: BTreeMap<TaskId, Waker>,
}

impl TaskQueue {
    pub fn new() -> Self {
        TaskQueue {
            tasks: BTreeMap::new(),
            task_queue: Arc::new(ArrayQueue::new(100)),
            waker_cache: BTreeMap::new(),
        }
    }

    pub fn spawn(&mut self, task: Task) {
        let id = task.id;
        // the task in tasks should have unique ID
        if self.tasks.insert(id, task).is_some() {
            panic!("task with same ID already in tasks");
        }
        if self.task_queue.push(id).is_err() {
            panic!("task_queue capacity exceeded");
        }
    }

    fn run_ready_task(&mut self){
        while let Some(id) = self.task_queue.pop() {

        let task = self.tasks.get_mut(&id).expect("task_queue and tasks are out of sync");
        let waker = self.waker_cache.entry(id).or_insert_with(|| TaskWaker::new(id, self.task_queue.clone())); //没有唤醒函数的话插入一个唤醒函数，但是我暂时不清楚唤醒函数是怎么产生的。
        let mut context = Context::from_waker(waker);
        match task.poll(&mut context) {
            Poll::Ready(_) => {
                self.waker_cache.remove(&id);
                self.tasks.remove(&id).expect("task_queue and tasks are out of sync");
            }
            Poll::Pending => {}
        }

    }
}
fn do_idle_sleep(&self) {
    use x86_64::instructions::interrupts::enable_and_hlt;
    interrupts::disable();
    if self.task_queue.is_empty() {
        enable_and_hlt();
    } else {
        interrupts::enable();
    }
}
pub fn run(&mut self) -> ! {
    loop {
        self.run_ready_task();
        self.do_idle_sleep();

    }
}
}

struct TaskWaker {
    task_id: TaskId,
    task_queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    fn wake_task(&self) {
        self.task_queue.push(self.task_id).expect("task_queue is full");
    }
    fn new(task_id: TaskId, task_queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(TaskWaker { task_id, task_queue }))
    }

}
impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task();
    }
}