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
