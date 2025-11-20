CREATE KEYSPACE IF NOT EXISTS visionary WITH REPLICATION = { 'class' : 'NetworkTopologyStrategy', 'replication_factor' : 3 };

USE "visionary";

CREATE TABLE IF NOT EXISTS accounts
(
    id             UUID PRIMARY KEY,
    username       TEXT,
    avatar         TEXT,
    email          TEXT,
    email_verified BOOLEAN,
    bio            TEXT,
    is_active      BOOLEAN,
    created_at     BIGINT,
    updated_at     BIGINT,
    deleted_at     BIGINT
);

CREATE TABLE IF NOT EXISTS sessions
(
    id           UUID PRIMARY KEY,
    account_id   UUID,
    jti          TEXT,
    expires_at   BIGINT,
    ip_address   TEXT,
    user_agent   TEXT,
    device_token TEXT,
    device_type  TEXT,
    created_at   BIGINT,
    updated_at   BIGINT,
    deleted_at   BIGINT
);

CREATE TABLE IF NOT EXISTS roles
(
    id         UUID PRIMARY KEY,
    account_id UUID,
    role_name  TEXT,
    created_at BIGINT,
    updated_at BIGINT,
    deleted_at BIGINT
);

CREATE TABLE IF NOT EXISTS providers
(
    id         UUID PRIMARY KEY,
    account_id UUID,
    provider   TEXT,
    identifier TEXT,
    created_at BIGINT,
    updated_at BIGINT,
    deleted_at BIGINT
);

CREATE TABLE IF NOT EXISTS experiences
(
    id           UUID PRIMARY KEY,
    account_id   UUID,
    technologies LIST<TEXT>,
    position     TEXT,
    created_at   BIGINT,
    updated_at   BIGINT,
    deleted_at   BIGINT
);

CREATE TABLE IF NOT EXISTS categories
(
    id         UUID PRIMARY KEY,
    name       TEXT,
    created_at BIGINT,
    updated_at BIGINT,
    deleted_at BIGINT
);

CREATE TABLE IF NOT EXISTS blogs
(
    id                  UUID PRIMARY KEY,
    author_id           UUID,
    category_id         UUID,
    name                TEXT,
    description         TEXT,
    markdown            TEXT,
    is_published        BOOLEAN,
    stars               INT,
    views               INT,
    estimated_read_time INT,
    created_at          BIGINT,
    updated_at          BIGINT,
    deleted_at          BIGINT
);

CREATE TABLE IF NOT EXISTS courses
(
    id          UUID PRIMARY KEY,
    author_id   UUID,
    category_id UUID,
    name        TEXT,
    description TEXT,
    level       TEXT,
    status      TEXT,
    rating      FLOAT,
    approximate INT,
    enrollments INT,
    created_at  BIGINT,
    updated_at  BIGINT,
    deleted_at  BIGINT
);

CREATE TABLE IF NOT EXISTS chapters
(
    id          UUID PRIMARY KEY,
    course_id   UUID,
    order_index INT,
    name        TEXT,
    description TEXT,
    created_at  BIGINT,
    updated_at  BIGINT,
    deleted_at  BIGINT
);

CREATE TABLE IF NOT EXISTS lessons
(
    id                UUID PRIMARY KEY,
    chapter_id        UUID,
    order_index       INT,
    name              TEXT,
    description       TEXT,
    markdown          TEXT,
    estimated_time_ms INT,
    created_at        BIGINT,
    updated_at        BIGINT,
    deleted_at        BIGINT
);

CREATE TABLE IF NOT EXISTS enrollments
(
    id           UUID PRIMARY KEY,
    account_id   UUID,
    course_id    UUID,
    status       TEXT,
    progress     FLOAT,
    completed_at BIGINT,
    created_at   BIGINT,
    updated_at   BIGINT,
    deleted_at   BIGINT
);

CREATE TABLE IF NOT EXISTS progresses
(
    id                UUID PRIMARY KEY,
    enrollment_id     UUID,
    lesson_id         UUID,
    is_completed      BOOLEAN,
    completed_at      BIGINT,
    time_spend_second INT,
    visits            INT,
    created_at        BIGINT,
    updated_at        BIGINT,
    deleted_at        BIGINT
);

CREATE TABLE IF NOT EXISTS bookmarks
(
    id         UUID PRIMARY KEY,
    account_id UUID,
    course_id  UUID,
    lesson_id  UUID,
    created_at BIGINT,
    updated_at BIGINT,
    deleted_at BIGINT
);
