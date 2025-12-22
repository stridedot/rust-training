fn main() -> anyhow::Result<()> {
    tonic_prost_build::configure()
        .out_dir("src/pb")
        .message_attribute(
            "user_stat.User",
            r#"#[derive(serde::Deserialize, serde::Serialize)]
            #[serde(rename_all = "snake_case")]
            #[derive(sqlx::FromRow)]
            "#,
        )
        .message_attribute(
            "user_stat.RawQueryRequest",
            r#"#[derive(serde::Deserialize, serde::Serialize)]
            #[serde(rename_all = "snake_case")]
            "#,
        )
        .message_attribute("user_stat.QueryRequest", r#""#)
        .message_attribute("user_stat.TimeQuery", r#""#)
        .message_attribute(
            "user_stat.IdQuery",
            r#"#[derive(serde::Deserialize, serde::Serialize)]
            #[serde(rename_all = "snake_case")]
            "#,
        )
        .compile_protos(
            &[
                "../protos/user-stat/message.proto",
                "../protos/user-stat/rpc.proto",
            ],
            &["../protos"],
        )?;

    Ok(())
}
