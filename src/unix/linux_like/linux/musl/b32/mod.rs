pub type c_long = i32;
pub type c_ulong = u32;
pub type nlink_t = u32;
pub type blksize_t = ::c_long;
pub type __u64 = ::c_ulonglong;
pub type __s64 = ::c_longlong;
pub type regoff_t = ::c_int;

s! {
    pub struct pthread_attr_t {
        __size: [u32; 9]
    }

    pub struct sigset_t {
        __val: [::c_ulong; 32],
    }

    pub struct msghdr {
        pub msg_name: u32, //*mut ::c_void,
        pub msg_namelen: ::socklen_t,
        pub msg_iov: u32, //*mut ::iovec,
        pub msg_iovlen: ::c_int,
        pub msg_control: u32, //*mut ::c_void,
        pub msg_controllen: ::socklen_t,
        pub msg_flags: ::c_int,
    }

    pub struct cmsghdr {
        pub cmsg_len: ::socklen_t,
        pub cmsg_level: ::c_int,
        pub cmsg_type: ::c_int,
    }

    pub struct sem_t {
        __val: [::c_int; 4],
    }
}

pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 32;
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 24;
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 20;

mod mips;
pub use self::mips::*;
