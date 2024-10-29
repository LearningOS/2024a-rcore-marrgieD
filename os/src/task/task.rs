//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    //add task_time
    pub task_time:TaskTime,
}
#[derive(Clone, Copy)]
pub struct TaskTime {
    /// The task status in it's lifecycle
    pub start_time: usize,
    /// The task context
    pub syscall_time: [u32; MAX_SYSCALL_NUM],
}

impl TaskTime {
    pub fn new() -> Self {
        TaskTime {
            start_time: 0,
            syscall_time: [0; MAX_SYSCALL_NUM],
        }
    }
}
/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}