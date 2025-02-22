//! This module provides the logic between the low-level abstract types and the types that the user will be interacting with.
//! You can think of the socket, connection management, congestion control.

pub use self::connection::{Connection, ConnectionEventAddress, ConnectionMessenger};
pub use self::events::SocketEvent;
pub use self::link_conditioner::LinkConditioner;
pub use self::virtual_connection::VirtualConnection;

mod connection;
mod connection_impl;
mod events;
mod link_conditioner;
mod virtual_connection;

pub mod constants;
