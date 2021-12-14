#![feature(get_mut_unchecked)]
pub mod cache;
pub mod dataset;
pub mod joader;
pub mod loader;
pub mod proto;
pub mod sampler;
pub mod service;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Role {
    Leader,
    Follower
}
