#![warn(missing_docs)]

#[macro_use]
extern crate downcast_rs as downcast;
#[cfg(feature = "native_lib")]
extern crate wayland_sys;
#[cfg(feature = "native_lib")]
use wayland_sys::common as syscom;

use std::os::raw::c_void;

use downcast::Downcast;

/// A group of messages
///
/// This represents a group of message that can be serialized on the protocol wire.
/// Typically the set of events or requests of a single interface.
///
/// Implementations of this trait are supposed to be
/// generated using the `wayland-scanner` crate.
pub trait MessageGroup: Sized {
    /// Whether this message is a destructor
    ///
    /// If it is, once send or receive the associated object cannot be used any more.
    fn is_destructor(&self) -> bool;
    #[cfg(feature = "native_lib")]
    /// Construct a message of this group from its C representation
    unsafe fn from_raw_c(obj: *mut c_void, opcode: u32, args: *const syscom::wl_argument)
        -> Result<Self, ()>;
    #[cfg(feature = "native_lib")]
    /// Build a C representation of this message
    ///
    /// It can only be accessed from the provided closure, and this consumes
    /// the message.
    fn as_raw_c_in<F, T>(self, f: F) -> T
    where
        F: FnOnce(u32, &mut [syscom::wl_argument]) -> T;
}

/// The description of a wayland interface
///
/// Implementations of this trait are supposed to be
/// generated using the `wayland-scanner` crate.
pub trait Interface: 'static {
    /// Set of requests associated to this interface
    ///
    /// Requests are messages from the client to the server
    type Requests: MessageGroup + 'static;
    /// Set of events associated to this interface
    ///
    /// Events are messages from the server to the client
    type Events: MessageGroup + 'static;
    /// Name of this interface
    const NAME: &'static str;
    #[cfg(feature = "native_lib")]
    /// Pointer to the C representation of this interface
    fn c_interface() -> *const ::syscom::wl_interface;
}

/// Trait representing implementations for wayland objects
///
/// Several wayland objects require you to act when some messages
/// are received. You program this act by providing an object
/// implementing this trait.
///
/// The trait requires a single method: `self.receive(message, metadata)`.
/// the `message` argument will often be an enum of the possible messages,
/// and the `metadata` argument contains associated information. Typically
/// an handle to the wayland object that received this message.
///
/// The trait is automatically implemented for `FnMut(Msg, Meta)` closures.
///
/// This is mostly used as a trait object in `wayland-client` and `wayland-server`,
/// and thus also provide methods providing `Any`-like downcasting functionnality.
/// See also the `downcast_impl` freestanding function.
pub trait Implementation<Meta, Msg>: Downcast {
    /// Receive a message
    fn receive(&mut self, msg: Msg, meta: Meta);
}

impl_downcast!(Implementation<Meta, Msg>);

impl<Meta, Msg, F> Implementation<Meta, Msg> for F
where
    F: FnMut(Msg, Meta) + 'static,
{
    fn receive(&mut self, msg: Msg, meta: Meta) {
        (self)(msg, meta)
    }
}

/// Attempt to downcast a boxed `Implementation` trait object.
///
/// Similar to `Box::<Any>::downcast()`.
pub fn downcast_impl<Msg: 'static, Meta: 'static, T: Implementation<Meta, Msg>>(
    b: Box<Implementation<Meta, Msg>>,
) -> Result<Box<T>, Box<Implementation<Meta, Msg>>> {
    if b.is::<T>() {
        unsafe {
            let raw: *mut Implementation<Meta, Msg> = Box::into_raw(b);
            Ok(Box::from_raw(raw as *mut T))
        }
    } else {
        Err(b)
    }
}

/// Anonymous interface
///
/// A special Interface implementation representing an
/// handle to an object for which the interface is not known.
pub struct AnonymousObject;

/// An empty enum representing a MessageGroup with no messages
pub enum NoMessage {}

impl Interface for AnonymousObject {
    type Requests = NoMessage;
    type Events = NoMessage;
    const NAME: &'static str = "";
    #[cfg(feature = "native_lib")]
    fn c_interface() -> *const ::syscom::wl_interface {
        ::std::ptr::null()
    }
}

impl MessageGroup for NoMessage {
    fn is_destructor(&self) -> bool {
        match *self {}
    }
    unsafe fn from_raw_c(
        _obj: *mut c_void,
        _opcode: u32,
        _args: *const syscom::wl_argument,
    ) -> Result<Self, ()> {
        Err(())
    }
    fn as_raw_c_in<F, T>(self, _f: F) -> T
    where
        F: FnOnce(u32, &mut [syscom::wl_argument]) -> T,
    {
        match self {}
    }
}
