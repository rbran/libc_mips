//! Definitions found commonly among almost all Unix derivatives
//!
//! More functions and definitions can be found in the more specific modules
//! according to the platform in question.

pub type c_schar = i8;
pub type c_uchar = u8;
pub type c_short = i16;
pub type c_ushort = u16;
pub type c_int = i32;
pub type c_uint = u32;
pub type c_float = f32;
pub type c_double = f64;
pub type c_longlong = i64;
pub type c_ulonglong = u64;
pub type intmax_t = i64;
pub type uintmax_t = u64;

pub type size_t = u32;
pub type ptrdiff_t = i32;
pub type intptr_t = i32;
pub type uintptr_t = u32;
pub type ssize_t = i32;

pub type pid_t = i32;
pub type in_addr_t = u32;
pub type in_port_t = u16;
pub type sighandler_t = ::size_t;
pub type cc_t = ::c_uchar;

pub type uid_t = u32;
pub type gid_t = u32;

missing! {
    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum DIR {}
}
pub type locale_t = u32; //*mut ::c_void;

s! {
    pub struct group {
        pub gr_name: u32, //*mut ::c_char,
        pub gr_passwd: u32, //*mut ::c_char,
        pub gr_gid: ::gid_t,
        pub gr_mem: u32, //*mut *mut ::c_char,
    }

    pub struct utimbuf {
        pub actime: time_t,
        pub modtime: time_t,
    }

    pub struct timeval {
        pub tv_sec: time_t,
        pub tv_usec: suseconds_t,
    }

    // linux x32 compatibility
    // See https://sourceware.org/bugzilla/show_bug.cgi?id=16437
    pub struct timespec {
        pub tv_sec: time_t,
        pub tv_nsec: ::c_long,
    }

    pub struct rlimit {
        pub rlim_cur: rlim_t,
        pub rlim_max: rlim_t,
    }

    pub struct rusage {
        pub ru_utime: timeval,
        pub ru_stime: timeval,
        pub ru_maxrss: c_long,
        pub ru_ixrss: c_long,
        pub ru_idrss: c_long,
        pub ru_isrss: c_long,
        pub ru_minflt: c_long,
        pub ru_majflt: c_long,
        pub ru_nswap: c_long,
        pub ru_inblock: c_long,
        pub ru_oublock: c_long,
        pub ru_msgsnd: c_long,
        pub ru_msgrcv: c_long,
        pub ru_nsignals: c_long,
        pub ru_nvcsw: c_long,
        pub ru_nivcsw: c_long,

        #[cfg(any(target_env = "musl", target_env = "ohos", target_os = "emscripten"))]
        __reserved: [c_long; 16],
    }

    pub struct ipv6_mreq {
        pub ipv6mr_multiaddr: in6_addr,
        pub ipv6mr_interface: ::c_uint,
    }

    pub struct hostent {
        pub h_name: u32, //*mut ::c_char,
        pub h_aliases: u32, //*mut *mut ::c_char,
        pub h_addrtype: ::c_int,
        pub h_length: ::c_int,
        pub h_addr_list: u32, //*mut *mut ::c_char,
    }

    pub struct iovec {
        pub iov_base: u32, //u32, //*mut ::c_void,
        pub iov_len: ::size_t,
    }

    pub struct pollfd {
        pub fd: ::c_int,
        pub events: ::c_short,
        pub revents: ::c_short,
    }

    pub struct winsize {
        pub ws_row: ::c_ushort,
        pub ws_col: ::c_ushort,
        pub ws_xpixel: ::c_ushort,
        pub ws_ypixel: ::c_ushort,
    }

    pub struct linger {
        pub l_onoff: ::c_int,
        pub l_linger: ::c_int,
    }

    pub struct sigval {
        // Actually a union of an int and a void*
        pub sival_ptr: u32, //*mut ::c_void
    }

    // <sys/time.h>
    pub struct itimerval {
        pub it_interval: ::timeval,
        pub it_value: ::timeval,
    }

    // <sys/times.h>
    pub struct tms {
        pub tms_utime: ::clock_t,
        pub tms_stime: ::clock_t,
        pub tms_cutime: ::clock_t,
        pub tms_cstime: ::clock_t,
    }

    pub struct servent {
        pub s_name: u32, //*mut ::c_char,
        pub s_aliases: u32, //*mut *mut ::c_char,
        pub s_port: ::c_int,
        pub s_proto: u32, //*mut ::c_char,
    }

    pub struct protoent {
        pub p_name: u32, //*mut ::c_char,
        pub p_aliases: u32, //*mut *mut ::c_char,
        pub p_proto: ::c_int,
    }
}

pub const INT_MIN: c_int = -2147483648;
pub const INT_MAX: c_int = 2147483647;

pub const SIG_DFL: sighandler_t = 0 as sighandler_t;
pub const SIG_IGN: sighandler_t = 1 as sighandler_t;
pub const SIG_ERR: sighandler_t = !0 as sighandler_t;
cfg_if! {
    if #[cfg(not(target_os = "nto"))] {
        pub const DT_UNKNOWN: u8 = 0;
        pub const DT_FIFO: u8 = 1;
        pub const DT_CHR: u8 = 2;
        pub const DT_DIR: u8 = 4;
        pub const DT_BLK: u8 = 6;
        pub const DT_REG: u8 = 8;
        pub const DT_LNK: u8 = 10;
        pub const DT_SOCK: u8 = 12;
    }
}
cfg_if! {
    if #[cfg(not(target_os = "redox"))] {
        pub const FD_CLOEXEC: ::c_int = 0x1;
    }
}

cfg_if! {
    if #[cfg(not(target_os = "nto"))]
    {
        pub const USRQUOTA: ::c_int = 0;
        pub const GRPQUOTA: ::c_int = 1;
    }
}
pub const SIGIOT: ::c_int = 6;

pub const S_ISUID: ::mode_t = 0x800;
pub const S_ISGID: ::mode_t = 0x400;
pub const S_ISVTX: ::mode_t = 0x200;

cfg_if! {
    if #[cfg(not(any(target_os = "haiku", target_os = "illumos",
                     target_os = "solaris")))] {
        pub const IF_NAMESIZE: ::size_t = 16;
        pub const IFNAMSIZ: ::size_t = IF_NAMESIZE;
    }
}

pub const LOG_EMERG: ::c_int = 0;
pub const LOG_ALERT: ::c_int = 1;
pub const LOG_CRIT: ::c_int = 2;
pub const LOG_ERR: ::c_int = 3;
pub const LOG_WARNING: ::c_int = 4;
pub const LOG_NOTICE: ::c_int = 5;
pub const LOG_INFO: ::c_int = 6;
pub const LOG_DEBUG: ::c_int = 7;

pub const LOG_KERN: ::c_int = 0;
pub const LOG_USER: ::c_int = 1 << 3;
pub const LOG_MAIL: ::c_int = 2 << 3;
pub const LOG_DAEMON: ::c_int = 3 << 3;
pub const LOG_AUTH: ::c_int = 4 << 3;
pub const LOG_SYSLOG: ::c_int = 5 << 3;
pub const LOG_LPR: ::c_int = 6 << 3;
pub const LOG_NEWS: ::c_int = 7 << 3;
pub const LOG_UUCP: ::c_int = 8 << 3;
pub const LOG_LOCAL0: ::c_int = 16 << 3;
pub const LOG_LOCAL1: ::c_int = 17 << 3;
pub const LOG_LOCAL2: ::c_int = 18 << 3;
pub const LOG_LOCAL3: ::c_int = 19 << 3;
pub const LOG_LOCAL4: ::c_int = 20 << 3;
pub const LOG_LOCAL5: ::c_int = 21 << 3;
pub const LOG_LOCAL6: ::c_int = 22 << 3;
pub const LOG_LOCAL7: ::c_int = 23 << 3;

cfg_if! {
    if #[cfg(not(target_os = "haiku"))] {
        pub const LOG_PID: ::c_int = 0x01;
        pub const LOG_CONS: ::c_int = 0x02;
        pub const LOG_ODELAY: ::c_int = 0x04;
        pub const LOG_NDELAY: ::c_int = 0x08;
        pub const LOG_NOWAIT: ::c_int = 0x10;
    }
}
pub const LOG_PRIMASK: ::c_int = 7;
pub const LOG_FACMASK: ::c_int = 0x3f8;

cfg_if! {
    if #[cfg(not(target_os = "nto"))]
    {
        pub const PRIO_MIN: ::c_int = -20;
        pub const PRIO_MAX: ::c_int = 20;
    }
}
pub const IPPROTO_ICMP: ::c_int = 1;
pub const IPPROTO_ICMPV6: ::c_int = 58;
pub const IPPROTO_TCP: ::c_int = 6;
pub const IPPROTO_UDP: ::c_int = 17;
pub const IPPROTO_IP: ::c_int = 0;
pub const IPPROTO_IPV6: ::c_int = 41;

pub const INADDR_LOOPBACK: in_addr_t = 2130706433;
pub const INADDR_ANY: in_addr_t = 0;
pub const INADDR_BROADCAST: in_addr_t = 4294967295;
pub const INADDR_NONE: in_addr_t = 4294967295;

pub const ARPOP_REQUEST: u16 = 1;
pub const ARPOP_REPLY: u16 = 2;

pub const ATF_COM: ::c_int = 0x02;
pub const ATF_PERM: ::c_int = 0x04;
pub const ATF_PUBL: ::c_int = 0x08;
pub const ATF_USETRAILERS: ::c_int = 0x10;

missing! {
    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum FILE {}
    #[cfg_attr(feature = "extra_traits", derive(Debug))]
    pub enum fpos_t {} // FIXME: fill this out with a struct
}

mod linux_like;
pub use self::linux_like::*;

pub use ffi::c_void;

mod align;
pub use self::align::*;
