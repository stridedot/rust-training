
-- sqlx database create -D postgres://postgres:123456@localhost:5432/stats
-- sqlx migrate add --source crm/migrations initial
-- qlx migrate run --source .\crm\migrations\ -D postgres://postgres:123456@localhost:5432/stats
create type gender as enum ('male', 'female', 'unknown');
create table if not exists user_stat (
    id bigserial primary key,
    name varchar(64),
    email varchar(255),
    gender gender default 'unknown',
    last_visited_at timestamptz,
    last_watched_at timestamptz,
    recent_watched int[],
    viewed_but_not_started int[],
    started_but_not_finished int[],
    finished int[],
    last_email_notification timestamptz,
    last_in_app_notification timestamptz,
    last_sms_notification timestamptz,
    created_at timestamptz default now()
);
