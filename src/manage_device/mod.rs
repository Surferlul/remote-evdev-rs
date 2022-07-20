use std::os::unix::io::AsRawFd;
use evdev::Device;
use nix::{
    fcntl::{FcntlArg, OFlag},
    sys::epoll,
};

pub mod serialize;
pub mod deserialize;

use crate::{
    epoll_struct::Epoll,
};

pub fn set_unblocking(device: &mut Device) {
    let raw_fd = device.as_raw_fd();
    // Set nonblocking
    nix::fcntl::fcntl(raw_fd, FcntlArg::F_SETFL(OFlag::O_NONBLOCK)).expect("Couldn't set nonblocking!");

    //Create epoll handle and attach raw_fd
    let epoll_fd = Epoll::new(epoll::epoll_create1(
        epoll::EpollCreateFlags::EPOLL_CLOEXEC,
    ).expect("Couldn't create epoll handle!"));
    let mut event = epoll::EpollEvent::new(epoll::EpollFlags::EPOLLIN, 0);
    epoll::epoll_ctl(
        epoll_fd.as_raw_fd(),
        epoll::EpollOp::EpollCtlAdd,
        raw_fd,
        Some(&mut event),
    ).expect("Couldn't attach raw_fd to epoll!");

    // We don't care about these, but the kernel wants to fill them.
    let _events = [epoll::EpollEvent::empty(); 2];

}