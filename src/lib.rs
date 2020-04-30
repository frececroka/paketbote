#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::{Config, Rocket};

pub mod error;
pub mod db;
pub mod web;

pub fn get_config() -> Config {
    Rocket::ignite().config().clone()
}
