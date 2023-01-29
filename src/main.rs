
use std::{thread::spawn};

use args::Args;
use clap::Parser;
use kill::{timeout_kill, kill_pid, Ans, State};
use nix::{
    libc::{dup2, execve,  fork, STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO, rusage, timeval, wait4, WSTOPPED, WIFSIGNALED, WTERMSIG, SIGUSR1, WEXITSTATUS, SIGSEGV, gettimeofday, timezone, PT_NULL},
};

mod args;
mod kill;

fn main() {
    let args = Args::parse();
    run_judge(args);
}
fn run_judge(args: Args) -> () {
    let mut start=timeval{ tv_sec: 0 , tv_usec: 0};
    let time_zone=PT_NULL as *mut timezone;
    unsafe{
        gettimeofday(&mut start,time_zone);
    }
    let child_pid = unsafe { fork() };
    if child_pid < 0 {
        panic!("fork 失败");
    } else if child_pid == 0 {
        // 子进程用来执行用户程序
        child_process(args);
        
    } else {
        //父进程用来监控子程序
        monitor(child_pid,args,start);
      
    }
}
fn child_process(args:Args){
    let (input,output,err)=args.get_file_fd();
    unsafe{
        dup2(input, STDIN_FILENO as i32);
        dup2(output, STDOUT_FILENO as i32);
        dup2(err, STDERR_FILENO as i32);
    }
  
    let (exe,arg,env)=args.load();
    args.set_resourse();
    unsafe{
        execve(exe, arg, env)
    };
}
fn monitor(pid:i32,arg:Args,start:timeval){
    spawn(move ||{
        timeout_kill(pid, arg.cpu_time_real)
    });
    let mut resource_usag: rusage = rusage {
        ru_utime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_stime: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        ru_maxrss: 0,
        ru_ixrss: 0,
        ru_idrss: 0,
        ru_isrss: 0,
        ru_minflt: 0,
        ru_majflt: 0,
        ru_nswap: 0,
        ru_inblock: 0,
        ru_oublock: 0,
        ru_msgsnd: 0,
        ru_msgrcv: 0,
        ru_nsignals: 0,
        ru_nvcsw: 0,
        ru_nivcsw: 0,
    };
    let mut status: i32 = 0;
    unsafe {
        if wait4(pid, &mut status, WSTOPPED, &mut resource_usag)==-1{
            kill_pid(pid)
        } 
    }
    let mut end=timeval{ tv_sec: 0 , tv_usec: 0};
    let time_zone=PT_NULL as *mut timezone;
    unsafe{
        gettimeofday(&mut end,time_zone);
    }
    let mut ans=Ans::default();
    ans.real_time=(end.tv_sec*1000+end.tv_usec/1000-start.tv_sec*1000-start.tv_usec/1000) as u64;
    //获取用户程序返回值
    if WIFSIGNALED(status) {
        ans.signal=WTERMSIG(status)
    }

    if ans.signal==SIGUSR1{
        ans.result=State::SystemError;
    }
    else{
        ans.exit_code=WEXITSTATUS(status);
        ans.cpu_time=(resource_usag.ru_utime.tv_sec*1000+resource_usag.ru_utime.tv_usec/1000) as u64;
        ans.memory=resource_usag.ru_maxrss as u64*1024;
        if ans.exit_code!=0{
            ans.result=State::RuntimeError;
        }
        if ans.signal==SIGSEGV{
            if ans.memory>arg.memory_size_max{
                ans.result=State::MemoryLimitExceeded;
            }else{
                ans.result=State::RuntimeError;
            }

        }else{
            if ans.signal!=0{
                ans.result=State::RuntimeError;
            }
            if ans.memory>arg.memory_size_max{
                ans.result=State::MemoryLimitExceeded;
            }
            if ans.real_time>arg.cpu_time_real {
                ans.result=State::RealTimeLimitExceeded;
            }
            if ans.cpu_time>arg.cpu_time_max{
                ans.result=State::CpuTimeLimitExceeded;
            }
        }
    }
    println!("{}",ans);
}