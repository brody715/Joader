#![feature(get_mut_unchecked)]
#![feature(cursor_remaining)]
#![feature(associated_type_defaults)]
pub mod cache;
pub mod dataset;
pub mod joader;
pub mod loader;
pub mod proto;
pub mod sampler;
pub mod service;
pub mod process;
pub mod local_cache;
pub mod sampler_bitmap;
pub mod service_sync;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Role {
    Leader,
    Follower
}
