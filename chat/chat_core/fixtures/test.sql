-- insert 3 workspaces
INSERT INTO workspace(name, owner_id)
  VALUES ('Takeaway', 1),
('Krispy', 0),
('Boxies', 0);

-- insert 5 users, all with hashed password '123456'
INSERT INTO users(username, email, password, workspace_id)
  VALUES ('Tyr Chen', 'tchen@acme.org', '$argon2id$v=19$m=19456,t=2,p=1$MxGhY+ib/kplwBPLa7u2ug$c5h9u7Sc8Px8J5+qgNdOjSY7ZJO2QN4rugKpapGW4XU', 1),
('Alice Chen', 'alice@acme.org', '$argon2id$v=19$m=19456,t=2,p=1$MxGhY+ib/kplwBPLa7u2ug$c5h9u7Sc8Px8J5+qgNdOjSY7ZJO2QN4rugKpapGW4XU', 1),
('Bob Chen', 'bob@acme.org', '$argon2id$v=19$m=19456,t=2,p=1$MxGhY+ib/kplwBPLa7u2ug$c5h9u7Sc8Px8J5+qgNdOjSY7ZJO2QN4rugKpapGW4XU', 1),
('Charlie Chen', 'charlie@acme.org', '$argon2id$v=19$m=19456,t=2,p=1$MxGhY+ib/kplwBPLa7u2ug$c5h9u7Sc8Px8J5+qgNdOjSY7ZJO2QN4rugKpapGW4XU', 1),
('Daisy Chen', 'daisy@acme.org', '$argon2id$v=19$m=19456,t=2,p=1$MxGhY+ib/kplwBPLa7u2ug$c5h9u7Sc8Px8J5+qgNdOjSY7ZJO2QN4rugKpapGW4XU', 1),
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

-- insert 4 chat
-- insert public/private channel
INSERT INTO chat(workspace_id, name, type, user_ids)
  VALUES (1, 'general', 'public_channel', '{1,2,3,4,5}'),
(1, 'private', 'private_channel', '{1,2,3}');

-- insert unnamed chat
INSERT INTO chat(workspace_id, type, user_ids)
  VALUES (1, 'single', '{1,2}'),
(1, 'group', '{1,3,4}');

INSERT INTO message(chat_id, sender_id, content)
  VALUES (1, 1, 'Hello, world!'),
(1, 2, 'Hi, there!'),
(1, 3, 'How are you?'),
(1, 4, 'I am fine, thank you!'),
(1, 5, 'Good to hear that!'),
(1, 1, 'Hello, world!'),
(1, 2, 'Hi, there!'),
(1, 3, 'How are you?'),
(1, 1, 'Hello, world!'),
(1, 1, 'Hello, world!');
