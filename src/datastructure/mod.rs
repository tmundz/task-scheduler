pub mod avl;
pub mod linklist;

/*
 * id to determine a task
 * rank to determine priority
    ll.push_back(task);
 * state will need to change to a different struct
*/
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    id: i32,
    rank: i32,
    state: i32, // will change to a task struct
}

impl Task {
    fn new(id: i32, rank: i32, state: i32) -> Task {
        Task { id, rank, state }
    }

    fn get_rank(&self) -> i32 {
        self.rank
    }
}
