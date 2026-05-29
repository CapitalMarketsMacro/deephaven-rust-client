//! Generated gRPC client stubs (tonic/prost) for the vendored Deephaven protos.
//!
//! The module nesting mirrors the proto package hierarchy
//! (`io.deephaven.proto.backplane.grpc` and `...script.grpc`) so that prost's
//! cross-package `super::` references resolve. Convenience re-exports are
//! provided at the bottom.

#![allow(clippy::all)]
#![allow(rustdoc::all)]
#![allow(missing_docs)]

pub mod io {
    pub mod deephaven {
        pub mod proto {
            pub mod backplane {
                pub mod grpc {
                    tonic::include_proto!("io.deephaven.proto.backplane.grpc");
                }
                pub mod script {
                    pub mod grpc {
                        tonic::include_proto!("io.deephaven.proto.backplane.script.grpc");
                    }
                }
            }
        }
    }
}

/// Types and client stubs from the `io.deephaven.proto.backplane.grpc` package.
pub use io::deephaven::proto::backplane::grpc;
/// Types and client stubs from the `io.deephaven.proto.backplane.script.grpc` package.
pub use io::deephaven::proto::backplane::script::grpc as script;
