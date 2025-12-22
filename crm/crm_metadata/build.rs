fn main() -> anyhow::Result<()> {
    tonic_prost_build::configure()
        .out_dir("src/pb")
        .compile_protos(
            &[
                "../protos/metadata/message.proto",
                "../protos/metadata/rpc.proto",
            ],
            &["../protos"],
        )?;
    Ok(())
}
