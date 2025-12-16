fn main() -> anyhow::Result<()> {
    tonic_prost_build::configure()
        .out_dir("src/pb")
        .message_attribute(
            "crm.WelcomeRequest",
            r#"#[derive(serde::Deserialize, serde::Serialize)]"#,
        )
        .message_attribute(
            "crm.WelcomeResponse",
            r#"#[derive(serde::Deserialize, serde::Serialize)]"#,
        )
        .message_attribute(
            "crm.RecallRequest",
            r#"#[derive(serde::Deserialize, serde::Serialize)]"#,
        )
        .message_attribute(
            "crm.RecallResponse",
            r#"#[derive(serde::Deserialize, serde::Serialize)]"#,
        )
        .message_attribute(
            "crm.RemindRequest",
            r#"#[derive(serde::Deserialize, serde::Serialize)]"#,
        )
        .message_attribute(
            "crm.RemindResponse",
            r#"#[derive(serde::Deserialize, serde::Serialize)]"#,
        )
        .compile_protos(
            &["../protos/crm/message.proto", "../protos/crm/rpc.proto"],
            &["../protos"],
        )?;

    Ok(())
}
