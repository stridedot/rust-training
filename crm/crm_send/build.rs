fn main() -> anyhow::Result<()> {
    tonic_prost_build::configure()
        .out_dir("src/pb")
        .compile_protos(
            &[
                "../protos/notification/message.proto",
                "../protos/notification/rpc.proto",
            ],
            &["../protos"],
        )?;

    Ok(())
}
