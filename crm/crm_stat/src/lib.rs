use std::{pin::Pin, sync::Arc};

use futures::Stream;
use sqlx::PgPool;
use tonic::{Request, Response, Status, async_trait};

use crate::{
    config::AppConfig,
    pb::user_stat::{
        QueryRequest, RawQueryRequest, User,
        user_stat_service_server::{UserStatService, UserStatServiceServer},
    },
};

pub mod abi;
pub mod config;
pub mod pb;

type ServiceResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<User, Status>> + Send>>;

pub struct UserStat {
    inner: Arc<UserStatInner>,
}

pub struct UserStatInner {
    #[allow(dead_code)]
    config: AppConfig,
    pg_pool: PgPool,
}

#[async_trait]
impl UserStatService for UserStat {
    #[doc = " Server streaming response type for the Query method."]
    type QueryStream = ResponseStream;

    #[doc = " Server streaming response type for the RawQuery method."]
    type RawQueryStream = ResponseStream;

    async fn query(&self, request: Request<QueryRequest>) -> ServiceResult<Self::QueryStream> {
        let req = request.into_inner();
        self.query(req).await
    }

    async fn raw_query(
        &self,
        request: Request<RawQueryRequest>,
    ) -> ServiceResult<Self::RawQueryStream> {
        let req = request.into_inner();
        self.raw_query(req).await
    }
}

impl UserStat {
    pub async fn try_new(config: AppConfig) -> anyhow::Result<Self> {
        let pool = PgPool::connect(&config.server.pg_url).await?;
        let inner = Arc::new(UserStatInner {
            config,
            pg_pool: pool,
        });

        Ok(Self { inner })
    }

    pub fn into_server(self) -> UserStatServiceServer<Self> {
        UserStatServiceServer::new(self)
    }
}

#[cfg(feature = "test-util")]
pub mod test_util {
    use super::*;
    use sqlx::{Executor, PgPool};

    impl UserStat {
        pub async fn test_new() -> anyhow::Result<(sqlx_db_tester::TestPg, Self)> {
            let config = AppConfig::load()?;

            let post = config.server.pg_url.rfind('/').expect("invalid pg_url");
            let server_url = &config.server.pg_url[..post];
            let (tdb, pool) = Self::get_test_pool(server_url).await;

            let state = Self {
                inner: Arc::new(UserStatInner {
                    config,
                    pg_pool: pool,
                }),
            };
            Ok((tdb, state))
        }

        pub async fn get_test_pool(server_url: &str) -> (sqlx_db_tester::TestPg, PgPool) {
            let tdb = sqlx_db_tester::TestPg::new(
                server_url.to_string(),
                std::path::Path::new("../migrations"),
            );
            println!("Using pg_url: {}, dbname: {}", tdb.server_url, tdb.dbname);
            let pool = tdb.get_pool().await;

            // run prepared sql to insert test data
            let sql = include_str!("../../fixtures/data.sql").split(';');
            let mut ts = pool.begin().await.expect("begin transaction failed");
            for s in sql {
                if s.trim().is_empty() {
                    continue;
                }
                ts.execute(s).await.expect("execute sql failed");
            }
            ts.commit().await.expect("commit transaction failed");

            (tdb, pool)
        }
    }
}
