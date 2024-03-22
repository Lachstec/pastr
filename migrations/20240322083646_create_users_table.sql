CREATE EXTENSION citext;

-- Domain that validates mail addresses via a regular expression
-- Not 100% correct, but works 90% of the time.
CREATE DOMAIN IF NOT EXISTS email AS citext
    CHECK ( value ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );

CREATE TABLE IF NOT EXISTS pastr.users(
    id uuid PRIMARY KEY,
    username TEXT NOT NULL,
    mail email NOT NULL,
    password_hash TEXT NOT NULL,
    
    UNIQUE(username),
    UNIQUE(mail),
);
