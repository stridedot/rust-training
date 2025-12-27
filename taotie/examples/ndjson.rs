use std::{fs::File, io::BufReader, path::PathBuf, sync::Arc, vec};

use arrow::{
    array::AsArray,
    datatypes::{DataType, Field, Schema, TimestampMicrosecondType},
    json::ReaderBuilder,
};
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc, offset::LocalResult};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct UserStat {
    email: String,
    name: String,
    gender: String,
    #[serde(deserialize_with = "deserialize_string_date_opt")]
    last_visited_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_string_date_opt")]
    last_watched_at: Option<DateTime<Utc>>,
    recent_watched: Vec<i32>,
    viewed_but_not_started: Vec<i32>,
    started_but_not_finished: Vec<i32>,
    finished: Vec<i32>,

    #[serde(deserialize_with = "deserialize_string_date_opt")]
    last_email_notification: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_string_date_opt")]
    last_in_app_notification: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_string_date_opt")]
    last_sms_notification: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_string_date")]
    created_at: DateTime<Utc>,
}

fn main() -> anyhow::Result<()> {
    let dt_type = DataType::Timestamp(arrow::datatypes::TimeUnit::Microsecond, None);
    let schema = Schema::new(vec![
        Field::new("email", DataType::Utf8, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("gender", DataType::Utf8, false),
        Field::new("last_visited_at", dt_type.clone(), true),
        Field::new("last_watched_at", dt_type.clone(), true),
        Field::new(
            "recent_watched",
            DataType::List(Arc::new(Field::new(
                "recent_watched",
                DataType::Int32,
                false,
            ))),
            true,
        ),
        Field::new(
            "viewed_but_not_started",
            DataType::List(Arc::new(Field::new(
                "viewed_but_not_started",
                DataType::Int32,
                false,
            ))),
            true,
        ),
        Field::new(
            "started_but_not_finished",
            DataType::List(Arc::new(Field::new(
                "started_but_not_finished",
                DataType::Int32,
                false,
            ))),
            true,
        ),
        Field::new(
            "finished",
            DataType::List(Arc::new(Field::new("finished", DataType::Int32, false))),
            true,
        ),
        Field::new("last_email_notification", dt_type.clone(), true),
        Field::new("last_in_app_notification", dt_type.clone(), true),
        Field::new("last_sms_notification", dt_type.clone(), true),
        Field::new("created_at", dt_type.clone(), false),
    ]);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("assets/users.ndjson");
    let reader = File::open(path)?;

    let reader = ReaderBuilder::new(Arc::new(schema))
        .with_batch_size(1024)
        .build(BufReader::new(reader))?;

    for batch in reader {
        let batch = batch?;
        // let email = batch.column(0).as_string::<i32>();
        // let name = batch.column(1).as_string::<i32>();

        // println!("{:?}", email.value(0));
        // println!("{:?}", name.value(0));

        // 映射到 UserStat
        // let data = Vec::new();
        // let mut writer = WriterBuilder::new()
        //     .with_explicit_nulls(true)
        //     .build::<_, JsonArray>(data);
        // writer.write_batches(&[&batch])?;
        // writer.finish()?;
        // let data = writer.into_inner();

        // let users = serde_json::from_slice::<Vec<UserStat>>(&data)?;
        // for user in users {
        //     println!("{:?}", user);
        // }

        // 直接从 batch 中取
        let emails = batch
            .column(batch.schema().index_of("email")?)
            .as_string::<i32>();
        let created_ats = batch
            .column(batch.schema().index_of("created_at")?)
            .as_primitive::<TimestampMicrosecondType>();

        for i in 0..batch.num_rows() {
            let email = emails.value(i);
            // Arrow Timestamp 内部存的是 i64 微秒，可以直接转为 chrono
            let ts_micros = created_ats.value(i);
            let dt = Utc.timestamp_micros(ts_micros).single();

            println!("Row {}: Email={}, CreatedAt={:?}", i, email, dt);
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn deserialize_string_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    let from = s
        .parse::<NaiveDateTime>()
        .map_err(serde::de::Error::custom)?;

    let date_time = match Utc.from_local_datetime(&from) {
        LocalResult::Single(dt) => dt,
        LocalResult::None => {
            return Err(serde::de::Error::custom("invalid date time format"));
        }
        LocalResult::Ambiguous(_, _) => {
            return Err(serde::de::Error::custom("invalid date time format"));
        }
    };

    Ok(date_time)
}

#[allow(dead_code)]
fn deserialize_string_date_opt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;

    match s {
        Some(s) => {
            let from = s
                .parse::<NaiveDateTime>()
                .map_err(serde::de::Error::custom)?;

            let date_time = match Utc.from_local_datetime(&from) {
                LocalResult::Single(dt) => dt,
                LocalResult::None => {
                    return Err(serde::de::Error::custom("invalid date time format"));
                }
                LocalResult::Ambiguous(_, _) => {
                    return Err(serde::de::Error::custom("invalid date time format"));
                }
            };

            Ok(Some(date_time))
        }
        None => Ok(None),
    }
}
