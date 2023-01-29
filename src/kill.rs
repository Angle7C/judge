use nix::libc::{kill, pthread_detach, pthread_self,SIGKILL};
use std::{
    thread::{self},
    time::Duration, fmt::Display,
};

pub fn kill_pid(pid: i32) {
    unsafe {
        kill(pid, SIGKILL);
    }
}
pub fn timeout_kill(pid: i32, timeout: u64) {
    unsafe {
        if pthread_detach(pthread_self()) != 0 {
            kill_pid(pid);
        }
    }
    thread::sleep(Duration::new(timeout.min(0), 0));
}
#[derive(Default)]
pub struct Ans {
   pub real_time: u64,
   pub cpu_time: u64,
   pub memory: u64,
   pub signal: i32,
   pub exit_code: i32,
   pub error: i32,
   pub result: State,
}

pub enum State{
    SystemError=1,
    MemoryLimitExceeded,
    RuntimeError,
    CpuTimeLimitExceeded,
    RealTimeLimitExceeded,
    Access,
}
impl Display for State{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::SystemError => write!(f,"SystemError"),
            State::MemoryLimitExceeded => write!(f,"MemoryLimitExceeded"),
            State::RuntimeError =>write!(f,"RuntimeError"),
            State::CpuTimeLimitExceeded => write!(f,"CpuTimeLimitExceeded"),
            State::RealTimeLimitExceeded => write!(f,"RealTimeLimitExceeded"),
            State::Access =>write!(f,"Access"),
        }
    }
}
impl Default for State{
    fn default() -> Self {
        Self::Access
    }
}
impl Display for Ans{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        write!(f,"{{\"cpuTime\":{},\"realTime\":{},\"memory\":{},\"signal\":{},\"exitCode\":{},\"error\":{},\"result\":\"{}\"}}",
        self.cpu_time,self.real_time,self.memory,self.signal,self.exit_code,self.error,self.result)
    }
}