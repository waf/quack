
extern crate libc;
extern crate slack;  // real-time messaging (rtm) client
extern crate slack_api; // web api and request / response models
extern crate config;
extern crate failure;

pub mod interface;
pub mod slack_integration;

mod implementation;
