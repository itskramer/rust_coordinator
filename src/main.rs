use std::sync::{Arc, Condvar, Mutex};
use std::collections::{HashMap, LinkedList};

/// master controller node.
/// children groups are each map to queue of locks
///

// enum LockState {
//     Free,
//     Locked,
// }

// struct Lock {
//     state: LockState
//     // queue: 
// }
//
struct DeployQueue {
    playbook: String,
    queue: LinkedList<DeployJob>,
}

struct DeployJob {
    status: String,
    name: String,
    triggered_by: String,
    args: String,
}

struct DeployCoordinator {
    locks: HashMap<String, DeployQueue>
}

fn main() {
}

