fn main() -> std::io::Result<()> {
    // tonic_build::configure()
    //     .type_attribute("routeguide.Point", "#[derive(Hash)]")
    //     .compile(&["proto/routeguide/route_guide.proto"], &["proto"])
    //     .unwrap();

    tonic_build::compile_protos("/home/jveiga/rust/dlog/proto/gogoproto/gogo.proto")?;
    tonic_build::compile_protos("/home/jveiga/rust/dlog/proto/log.proto")?;
    // tonic_build::compile_protos("/home/jveiga/rust/dlog/proto/log.proto").unwrap();
    // tonic_build::compile_protos("proto/echo/echo.proto").unwrap();

    Ok(())
}
