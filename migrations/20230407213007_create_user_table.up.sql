-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS
    users_user (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        NAME VARCHAR(50) NOT NULL,
        username VARCHAR(50) UNIQUE NOT NULL,
        PASSWORD VARCHAR(255) NOT NULL,
        email VARCHAR(254) UNIQUE NOT NULL,
        is_active BOOLEAN NOT NULL DEFAULT TRUE,
        is_staff BOOLEAN NOT NULL DEFAULT FALSE,
        is_superuser BOOLEAN NOT NULL DEFAULT FALSE,
        created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ,
        last_login TIMESTAMPTZ
    )