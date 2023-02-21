//! Global constants, such as compiler version used, algorithms, compression and filters supported and others

pub const COMPILATION_DATE: &str = "-";
pub const COMPILATION_TIME: &str = "-";
pub const NAME: &str = "-";
pub const VERSION: &str = "x.x.x";
pub const COMPILER: &str = "rustc";
pub const COMPILER_VERSION: &str = "x.x.x";
pub const HOST: &str = "-";
pub const TARGET: &str = "-";
pub const PROFILE: &str = "-";
pub const OPT_LEVEL: &str = "-";
pub const MAKEFLAGS: &str = "-";
pub const FEATURES: [&str; 1] = ["cpu"];
pub const PLATFORM_CPU_BITS: &str = "64";
pub const DEFAULT_THREAD_POOL_SIZE: usize = 1;
pub const MAX_THREAD_POOL_SIZE: usize = 1;
