-- Signup Request that stores the id of the unactivated user account.
-- Requests get deleted via trigger when they are older than 2 days.
CREATE TABLE IF NOT EXISTS pastr.users_confirmations (
    user_id uuid NOT NULL REFERENCES pastr.users(id),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);
-- Deletes rows older than 2 days via a trigger.
CREATE FUNCTION users_cofirmations_delete_expired() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN
DELETE FROM pastr.users
WHERE id IN (
        SELECT user_id
        FROM pastr.users_confirmations
        WHERE created_at < NOW() - INTERVAL '2 days'
    );
DELETE FROM pastr.users_confirmations
WHERE created_at < NOW() - INTERVAL '2 days';
RETURN NEW;
END;
$$;
CREATE TRIGGER users_confirmations_delete_expired_trigger
AFTER
INSERT ON pastr.users_confirmations EXECUTE PROCEDURE users_cofirmations_delete_expired();