use clap::arg;
use clap::command;
use clap::Parser;
use clap::ValueEnum;
use nix::libc::c_uint;
use nix::libc::rlimit;
use nix::libc::setrlimit;
use nix::sys::resource;
use std::fmt::Display;
use std::fs::File;
use std::os::fd::IntoRawFd;
#[derive(Debug, Parser, Clone, Copy, ValueEnum)]
pub enum Seccomp {
    Cpp,
    C,
    Other,
}

impl Display for Seccomp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Seccomp::Cpp => {
                write!(f, "Cpp")
            }
            Seccomp::C => {
                write!(f, "C")
            }
            Seccomp::Other => {
                write!(f, "Ohter")
            }
        }
    }
}
#[derive(Debug, Parser, Clone, Copy)]

pub struct Ans {
    cpu_time: u64,
    memory_size: u64,
    state: State,
}
#[derive(Debug, Parser, Clone, Copy, ValueEnum)]
pub enum State {
    AC,  //程序通过
    WA,  //答案错误
    TLE, //运行时间超出限制
    MLE, //内存超出限制,
    OLE, //输出内容超出限制
    RE,  //运行时错误
    CE,  //编译错误
}
#[derive(Debug, Parser, Clone)]
#[command(author,version,about,long_about=None)]
pub struct Args {
    #[arg(long = "cpu_max", default_value_t = 2)]
    pub cpu_time_max: u64,
    #[arg(long = "cpu_real", default_value_t = 6)]
    pub cpu_time_real: u64,
    #[arg(long="memory",default_value_t=1024*1024*1000)]
    pub memory_size_max: u64,
    #[arg(long="stack",default_value_t=1024*1024*1000)]
    pub stack_max: u64,
    #[arg(long = "process", default_value_t = 10)]
    pub process_number: u64,
    #[arg(long="outsize",default_value_t=1024*1024*1000*20)]
    pub output_size: u64,
    #[arg(long="exe",default_value_t=String::from(""))]
    pub exe_path: String,
    #[arg(long="input",default_value_t=String::from(""))]
    pub input_path: String,
    #[arg(long="out",default_value_t=String::from(""))]
    pub out_path: String,
    #[arg(long="error",default_value_t=String::from(""))]
    pub error_path: String,
    #[arg(long="log",default_value_t=String::from(""))]
    pub log_path: String,
    #[arg(long="mode",value_enum,default_value_t=Seccomp::Other)]
    pub seccomp: Seccomp,
}
impl Args {
    pub fn set_resourse(&self) {
        self.set_cpu();
        self.set_memory();
        self.set_stack();
        self.set_process();
        self.set_outsize();
    }

    pub fn get_file_fd(&self) -> (i32, i32, i32) {
        let input = File::open(self.input_path.as_str()).expect("无法打开input文件");
        let output = File::create(self.out_path.as_str()).expect("无法创建输出文件");
        let err = File::create(self.error_path.as_str()).expect("无法创建日志文件");
         (input.into_raw_fd(), output.into_raw_fd(), err.into_raw_fd()) 
    }
    pub fn load(&self) -> (*const i8, &*const i8, &*const i8) {
        let exe = self.exe_path.as_ptr() as *const i8;
        let arg = &{ 0 as *const i8 };
        let env = &{ 0 as *const i8 };
        (exe, arg, env)
    }
/// 设置CPU时间 单位 秒
fn set_cpu(&self) {
    let rlmit: rlimit = rlimit {
        rlim_cur: self.cpu_time_max,
        rlim_max: self.cpu_time_max,
    };
    let rlim: *const rlimit = &rlmit;
    unsafe {
        setrlimit(resource::Resource::RLIMIT_CPU as c_uint, rlim);
    }
}
/// 设置内存限制 单位 字节
fn set_memory(&self) {
    let rlmit: rlimit = rlimit {
        rlim_cur: self.memory_size_max,
        rlim_max: self.memory_size_max,
    };
    let rlim: *const rlimit = &rlmit;
    unsafe {
        setrlimit(resource::Resource::RLIMIT_AS as c_uint, rlim);
    }
}
/// 设置栈深度
fn set_stack(&self) {
    let rlmit: rlimit = rlimit {
        rlim_cur: self.stack_max,
        rlim_max: self.stack_max,
    };
    let rlim: *const rlimit = &rlmit;
    unsafe {
        setrlimit(resource::Resource::RLIMIT_STACK as c_uint, rlim);
    }
}
/// 设置进程数量
fn set_process(&self) {
    let rlmit: rlimit = rlimit {
        rlim_cur: self.process_number,
        rlim_max: self.process_number,
    };
    let rlim: *const rlimit = &rlmit;
    unsafe {
        setrlimit(resource::Resource::RLIMIT_NPROC as c_uint, rlim);
    }
}

/// 设置输出的最大大小 单位 字节
fn set_outsize(&self) {
    let rlmit: rlimit = rlimit {
        rlim_cur: self.output_size,
        rlim_max: self.output_size,
    };
    let rlim: *const rlimit = &rlmit;
    unsafe {
        setrlimit(resource::Resource::RLIMIT_FSIZE as c_uint, rlim);
    }
}
}


