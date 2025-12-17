use std::time::SystemTime;

use chrono::{DateTime, Days, Utc};
use fake::faker::chrono::en::DateTimeBetween;
use fake::{Dummy, Fake as _, Faker, Rng, faker::name::zh_cn};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, QueryBuilder};
use tokio::time::Instant;

#[derive(Clone, Debug, Dummy, Deserialize, Serialize, sqlx::Type)]
#[sqlx(type_name = "gender", rename_all = "lowercase")]
enum Gender {
    Male,
    Female,
    Unknown,
}

#[derive(Clone, Debug, Dummy, Deserialize, Serialize)]
struct UserStat {
    id: i64,
    #[dummy(faker = "zh_cn::Name()")]
    name: String,
    #[dummy(faker = "UniqueEmail")]
    email: String,
    gender: Gender,
    #[dummy(faker = "DateTimeBetween(start_at(30), end_at())")]
    last_visited_at: Option<DateTime<Utc>>,
    #[dummy(faker = "DateTimeBetween(start_at(60), end_at())")]
    last_watched_at: Option<DateTime<Utc>>,
    #[dummy(faker = "IntVec(0, 50, 1, 1000)")]
    recent_watched: Vec<i32>,
    #[dummy(faker = "IntVec(0, 100, 1, 100)")]
    viewed_but_not_started: Vec<i32>,
    #[dummy(faker = "IntVec(0, 15, 1, 100)")]
    started_but_not_finished: Vec<i32>,
    #[dummy(faker = "IntVec(0, 8, 1, 100)")]
    finished: Vec<i32>,
    #[dummy(faker = "DateTimeBetween(start_at(45), end_at())")]
    last_email_notification: Option<DateTime<Utc>>,
    #[dummy(faker = "DateTimeBetween(start_at(15), end_at())")]
    last_in_app_notification: Option<DateTime<Utc>>,
    #[dummy(faker = "DateTimeBetween(start_at(90), end_at())")]
    last_sms_notification: Option<DateTime<Utc>>,
    #[dummy(faker = "DateTimeBetween(start_at(365), end_at())")]
    created_at: DateTime<Utc>,
}

struct UniqueEmail;

impl Dummy<UniqueEmail> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &UniqueEmail, rng: &mut R) -> String {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis()
            % 1_000_000;
        let random = rng.random_range(100..=999);
        format!("user_{timestamp}{random}@example.com")
    }
}

fn start_at(days: u64) -> DateTime<Utc> {
    Utc::now().checked_sub_days(Days::new(days)).unwrap()
}

fn end_at() -> DateTime<Utc> {
    Utc::now()
}

struct IntVec<T>(usize, usize, T, T); // (最少个数，最多个数，最小值，最大值)

impl Dummy<IntVec<i32>> for Vec<i32> {
    fn dummy_with_rng<R: Rng + ?Sized>(init: &IntVec<i32>, rng: &mut R) -> Self {
        let IntVec(min_len, max_len, min_val, max_val) = init;
        let len = rng.random_range(*min_len..=*max_len);
        (0..len)
            .map(|_| rng.random_range(*min_val..=*max_val))
            .collect()
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pg = PgPool::connect("postgres://postgres:123456@localhost:5432/stats").await?;

    for i in 1..=500 {
        let users: Vec<UserStat> = (0..100).map(|_| Faker.fake()).collect();
        let start = Instant::now();
        raw_insert(users, &pg).await?;
        println!("Batch {} inserted in {:?}", i, start.elapsed());
    }

    Ok(())
}

async fn raw_insert(users: Vec<UserStat>, pool: &PgPool) -> anyhow::Result<()> {
    let mut qb = QueryBuilder::new(
        r#"
        insert into user_stat (
            name,
            email,
            gender,
            last_visited_at,
            last_watched_at,
            recent_watched,
            viewed_but_not_started,
            started_but_not_finished,
            finished,
            last_email_notification,
            last_in_app_notification,
            last_sms_notification,
            created_at
        )
    "#,
    );

    qb.push_values(users, |mut b, u| {
        b.push_bind(u.name)
            .push_bind(u.email)
            .push_bind(u.gender)
            .push_bind(u.last_visited_at)
            .push_bind(u.last_watched_at)
            .push_bind(u.recent_watched)
            .push_bind(u.viewed_but_not_started)
            .push_bind(u.started_but_not_finished)
            .push_bind(u.finished)
            .push_bind(u.last_email_notification)
            .push_bind(u.last_in_app_notification)
            .push_bind(u.last_sms_notification)
            .push_bind(u.created_at);
    });

    qb.build().execute(pool).await?;

    Ok(())
}
