CREATE TABLE IF NOT EXISTS user (
      id          TEXT    NOT NULL
    , name        TEXT    NOT NULL
    , email       TEXT    NOT NULL
    , salt        TEXT    NOT NULL
    , hash        TEXT    NOT NULL

    , PRIMARY KEY (id)
    , UNIQUE (name) ON CONFLICT FAIL
) STRICT, WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS session (
      id          TEXT    NOT NULL
    , user_id     TEXT    NOT NULL
    , token       TEXT    NOT NULL
    , expiry      INT     NOT NULL
    , client      INT     NOT NULL    DEFAULT TRUE

    , PRIMARY KEY   (id)
    , FOREIGN KEY   (user_id)   REFERENCES  user(id)    ON DELETE CASCADE
    , UNIQUE        (token)     ON CONFLICT FAIL
) STRICT, WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS bookmark (
      id          TEXT    NOT NULL
    , user_id     TEXT    NOT NULL
    , url         TEXT    NOT NULL
    , title       TEXT    NOT NULL
    , about       TEXT
    , star        INT     NOT NULL    DEFAULT FALSE
    , archive     INT     NOT NULL    DEFAULT FALSE
    , trash       INT     NOT NULL    DEFAULT FALSE

    , PRIMARY KEY   (id)
    , FOREIGN KEY   (user_id)       REFERENCES  user(id)    ON DELETE CASCADE
    , UNIQUE        (user_id, url)  ON CONFLICT FAIL
) STRICT, WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS tag (
      id          TEXT    NOT NULL
    , name        TEXT    NOT NULL

    , PRIMARY KEY   (id)
    , UNIQUE        (name)  ON CONFLICT IGNORE
) STRICT, WITHOUT ROWID;

CREATE TABLE IF NOT EXISTS bookmark_tag_link (
      bookmark_id TEXT    NOT NULL
    , tag_id      TEXT    NOT NULL

    , FOREIGN KEY   (bookmark_id)           REFERENCES  bookmark(id)    ON DELETE CASCADE
    , FOREIGN KEY   (tag_id)                REFERENCES  tag(id)         ON DELETE CASCADE
    , UNIQUE        (bookmark_id, tag_id)   ON CONFLICT IGNORE
) STRICT;

CREATE TABLE IF NOT EXISTS settings (
      user_id   TEXT    NOT NULL
    , theme     INT     NOT NULL    DEFAULT FALSE
    , home      TEXT    NOT NULL    DEFAULT 'all'

    , FOREIGN KEY   (user_id)   REFERENCES  user(id)    ON DELETE CASCADE
    , UNIQUE        (user_id)   ON CONFLICT REPLACE
    , CHECK         (home IN ('all', 'inbox'))
) STRICT;
