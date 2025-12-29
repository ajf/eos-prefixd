mod arista_session;
mod paths;
mod session;

pub use session::Session;

pub mod gnmi {
    tonic::include_proto!("gnmi");
}

pub mod gnmi_ext {
    tonic::include_proto!("gnmi_ext");
}

