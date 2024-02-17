// Code used from pam ()
pub use pam_sys as ffi;

pub mod conv;
pub mod enums;
pub mod env;
pub mod functions;
pub mod module;
pub mod types;

pub use crate::misc::{enums::*, functions::*, types::*};

#[cfg(feature = "client")]
pub mod client;

#[cfg(feature = "module")]
pub mod module;

pub use crate::misc::{
    conv::{Conversation, SiweConv},
    enums::*,
};

#[cfg(feature = "client")]
pub use client::Client;

#[cfg(feature = "module")]
pub use module::PamModule;