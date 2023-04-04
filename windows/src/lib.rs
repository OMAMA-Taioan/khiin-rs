#![cfg(windows)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate rust_i18n;
i18n!("locales");

// temporary
// #![allow(unused)]

mod config;
mod dll;
mod geometry;
mod reg;
mod resource;
mod tip;
mod ui;
mod utils;
