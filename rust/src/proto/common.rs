#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(enumeration = "status::Code", tag = "1")]
    pub code: i32,
}
/// Nested message and enum types in `Status`.
pub mod status {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Code {
        Ok = 0,
        False = 1,
    }
}
