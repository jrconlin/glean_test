#![warn(rust_2018_idioms)]
#![allow(clippy::try_err)]

#[macro_use]
extern crate slog_scope;

#[macro_use]
pub mod logging;
pub mod error;
pub mod metrics;
pub mod server;
pub mod settings;
pub mod tags;
pub mod web;

pub mod glean_metrics;
