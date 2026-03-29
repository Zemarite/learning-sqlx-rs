
-- migrations/000001_create_organization_division_group_member.up.sql

CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Organizations (top-level)
CREATE TABLE organizations (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),  -- or use app-generated v7 for better concurrency across distributed systems
    name            TEXT NOT NULL,
    description     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Divisions (must belong to an Organization, must have ≥1 member)
CREATE TABLE divisions (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    description     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE(organization_id, name)  -- optional: division names unique per org
);

-- Groups (can belong to Organization OR Division, must have ≥1 member)
CREATE TABLE groups (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID REFERENCES organizations(id) ON DELETE CASCADE,
    division_id     UUID REFERENCES divisions(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,
    description     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CHECK (
        (organization_id IS NOT NULL AND division_id IS NULL) OR
        (organization_id IS NULL AND division_id IS NOT NULL) OR
        (organization_id IS NOT NULL AND division_id IS NOT NULL)  -- allow both if needed
    ),  -- At least one parent required; adjust CHECK as per exact rules

    UNIQUE(organization_id, division_id, name)  -- prevent duplicate names in same context
);

-- Members (belong to Organization; optionally to one or more Divisions and many Groups)
CREATE TABLE members (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    division_id     UUID REFERENCES divisions(id) ON DELETE SET NULL,  -- optional division
    name            TEXT NOT NULL,
    email           TEXT UNIQUE,  -- or other unique identifier
    role            TEXT,         -- e.g., "employee", "volunteer"
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Many-to-many: Members ↔ Groups
CREATE TABLE member_groups (
    member_id UUID NOT NULL REFERENCES members(id) ON DELETE CASCADE,
    group_id  UUID NOT NULL REFERENCES groups(id) ON DELETE CASCADE,
    joined_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (member_id, group_id)
);

-- Indexes for performance
CREATE INDEX idx_divisions_organization_id ON divisions(organization_id);
CREATE INDEX idx_groups_organization_id ON groups(organization_id);
CREATE INDEX idx_groups_division_id ON groups(division_id);
CREATE INDEX idx_members_organization_id ON members(organization_id);
CREATE INDEX idx_members_division_id ON members(division_id);
CREATE INDEX idx_member_groups_group_id ON member_groups(group_id);

