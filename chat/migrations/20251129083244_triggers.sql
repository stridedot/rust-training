-- Add migration script here
-- if chat changed, notify with chat data
CREATE OR REPLACE FUNCTION add_to_chat()
  RETURNS TRIGGER
  AS $$
BEGIN
  RAISE NOTICE 'add_to_chat: %', NEW;
  PERFORM
    pg_notify('chat_updated', json_build_object('op', TG_OP, 'old', OLD, 'new', NEW)::text);
  RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER add_to_chat_trigger
  AFTER INSERT OR UPDATE OR DELETE ON chat
  FOR EACH ROW
  EXECUTE FUNCTION add_to_chat();

-- if new message added, notify with message data
CREATE OR REPLACE FUNCTION add_to_message()
  RETURNS TRIGGER
  AS $$
DECLARE
  USER_IDS bigint[];
BEGIN
  IF TG_OP = 'INSERT' THEN
    RAISE NOTICE 'add_to_message: %', NEW;
    SELECT
      c.user_ids INTO USER_IDS
    FROM
      chat as c
    WHERE
      c.id = NEW.chat_id;
    PERFORM
      pg_notify('chat_message_created', json_build_object('message', NEW, 'user_ids', USER_IDS)::text);
  END IF;
  RETURN NEW;
END;
$$
LANGUAGE plpgsql;

CREATE TRIGGER add_to_message_trigger
  AFTER INSERT ON message
  FOR EACH ROW
  EXECUTE FUNCTION add_to_message();
