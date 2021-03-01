use wayland_commons::Interface;

pub mod client;
pub mod server;

mod debug;
mod map;
mod socket;
mod wire;

#[inline]
fn nix_to_io(e: nix::Error) -> std::io::Error {
    if let nix::Error::Sys(errno) = e {
        errno.into()
    } else {
        panic!("Unexpected nix error: {:?}", e);
    }
}

#[inline]
fn same_interface(a: &'static Interface, b: &'static Interface) -> bool {
    a as *const Interface == b as *const Interface || a.name == b.name
}
