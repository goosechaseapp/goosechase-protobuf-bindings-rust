/// This is the module to emulate the structure of the protobuf file imports to
/// prevent any build issues.

pub mod data {
    pub mod email {
        use serde::{Deserialize, Serialize};
        tonic::include_proto!("goosechase.data.email");
    }

    pub use email::*;
}
