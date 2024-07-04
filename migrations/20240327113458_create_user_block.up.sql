-- status: 0 = unblocked, 1 = blocked
CREATE TABLE IF NOT EXISTS
    user_block (
        id UUID PRIMARY KEY DEFAULT uuid_generate_v4 (),
        blocker_id UUID REFERENCES users_user(id) NOT NULL,
        blocked_id UUID REFERENCES users_user(id) NOT NULL,
        blocked_status INTEGER NOT NULL DEFAULT 1 CHECK (blocked_status IN (0, 1)),
        blocked_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
        unblocked_at TIMESTAMPTZ,
        UNIQUE (blocker_id, blocked_id)
    );