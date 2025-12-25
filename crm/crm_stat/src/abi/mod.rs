use std::collections::HashMap;

use chrono::{DateTime, SecondsFormat, TimeZone as _, Utc};
use prost_types::Timestamp;
use tonic::{Response, Status};

use crate::{
    ResponseStream, ServiceResult, UserStat,
    pb::user_stat::{QueryRequest, RawQueryRequest, TimeQuery, User},
};

#[derive(Debug)]
pub enum SqlBuildError {
    InvalidTimestamp,
}

impl From<SqlBuildError> for Status {
    fn from(err: SqlBuildError) -> Self {
        match err {
            SqlBuildError::InvalidTimestamp => Status::invalid_argument("invalid timestamp"),
        }
    }
}

impl UserStat {
    pub async fn query(&self, req: QueryRequest) -> ServiceResult<ResponseStream> {
        let sql = req.to_sql()?;
        tracing::info!("query: {}", sql);
        let raw_query = RawQueryRequest { query: sql };

        self.raw_query(raw_query).await
    }

    pub async fn raw_query(&self, req: RawQueryRequest) -> ServiceResult<ResponseStream> {
        let Ok(ret) = sqlx::query_as::<_, User>(&req.query)
            .fetch_all(&self.inner.pg_pool)
            .await
        else {
            return Err(Status::internal(format!("raw query failed: {}", req.query)));
        };

        Ok(Response::new(Box::pin(futures::stream::iter(
            ret.into_iter().map(Ok),
        ))))
    }
}

impl QueryRequest {
    fn to_sql(&self) -> Result<String, SqlBuildError> {
        let mut sql = "select email, name from user_stat where 1=1".to_string();

        for (name, query) in &self.timestamps {
            let q = timestamp_query(name, query)?;
            sql.push_str(&q);
        }

        for (name, ids) in &self.ids {
            let q = ids_query(name, &ids.ids);
            sql.push_str(&q);
        }

        tracing::info!("generated sql: {}", sql);

        Ok(sql)
    }
}

fn to_rfc3339_opts(ts: &Timestamp) -> Result<String, SqlBuildError> {
    let dt = Utc
        .timestamp_opt(ts.seconds, ts.nanos as _)
        .single()
        .ok_or(SqlBuildError::InvalidTimestamp)?;

    Ok(dt.to_rfc3339_opts(SecondsFormat::Secs, true))
}

fn timestamp_query(name: &str, query: &TimeQuery) -> Result<String, SqlBuildError> {
    let mut sql = String::new();

    if let Some(lower) = &query.lower {
        let lower = to_rfc3339_opts(lower)?;
        sql.push_str(&format!(" and {} >= '{}'", name, lower));
    }
    if let Some(upper) = &query.upper {
        let upper = to_rfc3339_opts(upper)?;
        sql.push_str(&format!(" and {} <= '{}'", name, upper));
    }

    Ok(sql)
}

fn ids_query(name: &str, ids: &[u32]) -> String {
    if ids.is_empty() {
        return String::new();
    }

    format!(" and array{:?} <@ {}", ids, name)
}

impl QueryRequest {
    pub fn new_with_dt(field: &str, d1: DateTime<Utc>, d2: DateTime<Utc>) -> Self {
        let ts1 = Timestamp {
            seconds: d1.timestamp(),
            nanos: 0,
        };

        let ts2 = Timestamp {
            seconds: d2.timestamp(),
            nanos: 0,
        };

        let tq = TimeQuery {
            lower: Some(ts1),
            upper: Some(ts2),
        };

        Self {
            timestamps: HashMap::from([(field.to_string(), tq)]),
            ids: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use futures::StreamExt as _;

    use crate::pb::user_stat::IdQuery;

    use super::*;

    #[test]
    fn test_ids_query() {
        let ids = vec![1, 2, 3];
        let q = ids_query("ids", &ids);

        assert_eq!(q, " and array[1, 2, 3] <@ ids");
    }

    #[test]
    fn test_timestamp_query() {
        let query = TimeQuery {
            lower: Some(Timestamp {
                seconds: 1672531200,
                nanos: 0,
            }),
            upper: Some(Timestamp {
                seconds: 1672617600,
                nanos: 0,
            }),
        };
        let q = timestamp_query("created_at", &query).unwrap();

        assert_eq!(
            q,
            " and created_at >= '2023-01-01T00:00:00Z' and created_at <= '2023-01-02T00:00:00Z'"
        );
    }

    fn normalize_sql(s: &str) -> String {
        s.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }

    #[test]
    fn test_query_request_to_sql() -> Result<(), SqlBuildError> {
        let mut m = HashMap::new();
        m.insert(
            "created_at".to_string(),
            TimeQuery {
                lower: Some(Timestamp {
                    seconds: 1672531200,
                    nanos: 0,
                }),
                upper: Some(Timestamp {
                    seconds: 1672617600,
                    nanos: 0,
                }),
            },
        );
        let mut m2 = HashMap::new();
        m2.insert("user_ids".to_string(), IdQuery { ids: vec![1, 2, 3] });

        let req = QueryRequest {
            timestamps: m,
            ids: m2,
        };
        let sql = req.to_sql()?;

        assert_eq!(
            normalize_sql(&sql),
            normalize_sql(
                r#"
                select email, name from user_stat
                where 1=1
                and created_at >= '2023-01-01T00:00:00Z'
                and created_at <= '2023-01-02T00:00:00Z'
                and array[1, 2, 3] <@ user_ids
                "#
            )
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_raw_query_should_work() -> Result<(), anyhow::Error> {
        let (_tdb, state) = UserStat::test_new().await?;

        let req: RawQueryRequest = RawQueryRequest {
            query: "select email, name from user_stat where created_at >= '2023-01-01T00:00:00Z'"
                .to_string(),
        };

        let mut stream = state.raw_query(req).await?.into_inner();

        while let Some(res) = stream.next().await {
            println!("user: {:?}", res);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_query_should_work() -> Result<(), anyhow::Error> {
        let (_tdb, state) = UserStat::test_new().await?;

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

        let mut stream = state.query(req).await?.into_inner();

        while let Some(res) = stream.next().await {
            println!("user: {:?}", res);
        }

        Ok(())
    }
}
