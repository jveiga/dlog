fn main() -> std::io::Result<()> {
    let mut config = prost_build::Config::new();
    config.type_attribute("Record", "#[derive(Serialize)");
    prost_build::compile_protos(&["src/log.proto"], &["src/"])?;

    // tonic_build::compile_protos("/home/jveiga/rust/dlog/proto/gogoproto/gogo.proto")?;
    // tonic_build::compile_protos("/home/jveiga/rust/dlog/proto/log.proto")?;
    // tonic_build::compile_protos("/home/jveiga/rust/dlog/proto/log.proto").unwrap();
    // tonic_build::compile_protos("proto/echo/echo.proto").unwrap();

    Ok(())
}
