use std::{collections::HashMap, net::SocketAddr};

use anyhow::Result;
use crm_stat::{
    UserStat,
    pb::user_stat::{
        QueryRequest, RawQueryRequest, TimeQuery, user_stat_service_client::UserStatServiceClient,
    },
};
use futures::StreamExt as _;
use prost_types::Timestamp;
use sqlx_db_tester::TestPg;
use tonic::transport::Server;

const PORT_BASE: u32 = 60002;

#[tokio::test]
async fn test_raw_query_should_work() -> Result<()> {
    let (_tdb, addr) = start_server(PORT_BASE).await?;

    let mut client = UserStatServiceClient::connect(format!("http://{}", addr)).await?;

    let req = RawQueryRequest {
        query: "SELECT * FROM user_stat WHERE created_at > '2024-01-01' LIMIT 5".to_string(),
    };

    let resp = client.raw_query(req).await?.into_inner();
    let ret = resp.collect::<Vec<_>>().await;

    for item in ret {
        println!("item: {:?}", item?);
    }

    Ok(())
}

#[tokio::test]
async fn test_query_should_fail() -> Result<()> {
    let (_tdb, addr) = start_server(PORT_BASE).await?;

    let mut client = UserStatServiceClient::connect(format!("http://{}", addr)).await?;

    let mut m = HashMap::new();
    m.insert(
        "created_at".to_string(),
        TimeQuery {
            lower: Some(Timestamp {
                seconds: 1672531200,
                nanos: 0,
            }),
            upper: Some(Timestamp {
                seconds: 1892617600,
                nanos: 0,
            }),
        },
    );
    let m2 = HashMap::new();

    let req = QueryRequest {
        timestamps: m,
        ids: m2,
    };

    let resp = client.query(req).await?;
    let ret = resp.into_inner().collect::<Vec<_>>().await;

    for item in ret {
        println!("item: {:?}", item?);
    }

    Ok(())
}

async fn start_server(port: u32) -> Result<(TestPg, SocketAddr)> {
    let addr = format!("[::1]:{}", port).parse()?;
    let (tdb, svc) = UserStat::test_new().await?;

    tokio::spawn(async move {
        Server::builder()
            .add_service(svc.into_server())
            .serve(addr)
            .await
            .unwrap();
    });

    Ok((tdb, addr))
}
