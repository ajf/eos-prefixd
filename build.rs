fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=vendor/openconfig-gnmi/proto/gnmi/gnmi.proto");
    println!("cargo:rerun-if-changed=vendor/openconfig-gnmi/proto/gnmi_ext/gnmi_ext.proto");

    tonic_build::configure()
        .build_server(false)
        .compile_protos(
            &[
                "vendor/openconfig-gnmi/proto/gnmi/gnmi.proto",
                "vendor/openconfig-gnmi/proto/gnmi_ext/gnmi_ext.proto",
            ],
            &["vendor/openconfig-gnmi/proto"],
        )?;

    Ok(())
}
