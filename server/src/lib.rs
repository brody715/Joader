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


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Role {
    Leader,
    Follower
}
