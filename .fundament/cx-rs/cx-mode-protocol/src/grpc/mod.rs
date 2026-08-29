#[cfg(cx_bazel)]
pub use code_mode_proto::cx::code_mode::v1::*;

#[cfg(not(cx_bazel))]
tonic::include_proto!("cx.code_mode.v1");

pub const MAX_IDENTIFIER_BYTES: usize = 256;
