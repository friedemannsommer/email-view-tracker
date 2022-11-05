CREATE TABLE users
(
    created_at TIMESTAMP NOT NULL,
    id         BINARY    NOT NULL PRIMARY KEY,
    name       TEXT      NOT NULL,
    password   TEXT      NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

CREATE TABLE trackers
(
    created_at TIMESTAMP       NOT NULL,
    id         BINARY          NOT NULL PRIMARY KEY,
    name       TEXT            NOT NULL,
    updated_at TIMESTAMP       NOT NULL,
    user_id    BINARY          NOT NULL,
    views      BIGINT UNSIGNED NOT NULL,
    INDEX uid (user_id),
    FOREIGN KEY (user_id) REFERENCES users (id)
        ON UPDATE NO ACTION
        ON DELETE CASCADE
);
