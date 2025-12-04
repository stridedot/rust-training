-- Add migration script here
-- Add migration script here
-- this file is used for postgresql database initialization

-- create workspace
drop table if exists workspace;
create table workspace (
  id bigserial primary key,
  name varchar(128) not null default '',
  owner_id bigint not null default 0,
  created_at timestamptz default current_timestamp
);
insert into workspace (name) values ('Takeaway'), ('Krispy'), ('Boxies'), ('Snackish'), ('Orderdine'), ('Bevbox'), ('Mealprepper'), ('Tasteful');

-- create user table
drop table if exists users;
create table users (
  id bigserial primary key,
  username varchar(64) not null default '',
  email varchar(64) not null default '',
  -- hashed argon2 password
  password varchar(255) not null default '',
  workspace_id bigint not null default 0,
  created_at timestamptz default current_timestamp
);
insert into users (username, email, password, workspace_id) values
('Noah', 'Noah@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1),
('Liam', 'Liam@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1),
('Mason', 'Mason@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1),
('Jacob', 'Jacob@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1),
('William', 'William@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1),
('Ethan', 'Ethan@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1),
('Michael', 'Michael@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1),
('Alexander', 'Alexander@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1),
('Emma', 'Emma@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1),
('Olivia', 'Olivia@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1),
('Sophia', 'Sophia@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1),
('Isabella', 'Isabella@gmail.com', '$argon2id$v=19$m=19456,t=2,p=1$YuupYiBUSY6Ge+3BiDlF5A$lecghI2716CgzM2aixxKx9wibZWDasOvDcHS3BE+M5I', 1);


-- create chat type: single, group, private_channel, public_channel
drop type if exists chat_type cascade;
create type chat_type as enum (
  'single',          -- 单聊（一对一聊天）
  'group',           -- 群聊（多人聊天）
  'private_channel', -- 私有频道（需邀请加入）
  'public_channel'   -- 公开频道（所有人可加入）
);

-- create chat table
drop table if exists chat;
create table chat (
  id bigserial primary key,
  name varchar(128) not null default '',
  type chat_type not null,
  workspace_id bigint not null default 0,
  user_ids bigint[] not null default '{}',
  created_at timestamptz default current_timestamp
);

-- create message table
drop table if exists message;
create table message (
  id bigserial primary key,
  chat_id bigint not null,
  sender_id bigint not null,
  content text not null default '',
  files text[] not null default '{}',
  created_at timestamptz default current_timestamp
);

CREATE INDEX idx_chat_created_at ON message(chat_id, created_at);
CREATE INDEX idx_sender_id ON message(sender_id);
