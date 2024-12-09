struct Proc {
    pid: i32,
    all: String,
    state: String,
    vmsize: String,
    vmrss: String,
    vmdata: String,
    vmexe: String,
}

pub fn accept_proc() {}

pub fn close_proc() {}
