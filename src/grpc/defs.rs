#[allow(clippy::unwrap_used)]
pub mod grpc_service {
    pub mod v1 {
        include!("../proto/api_service.v1.rs");
        include!("../proto/api_service.v1.serde.rs");
    }
}

pub mod open_graph_def {
    pub mod v1 {
        include!("../proto/open_graph_def.v1.rs");
        include!("../proto/open_graph_def.v1.serde.rs");
    }
}
