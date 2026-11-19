-- Add migration script here
CREATE SCHEMA IF NOT EXISTS users;

CREATE TABLE users.main (
    id UUID PRIMARY KEY DEFAULT uuidv7() NOT NULL, 
    username VARCHAR(100) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ 
  );

CREATE TABLE users.detail (
    user_id UUID PRIMARY KEY REFERENCES users.main(id) ON DELETE CASCADE,
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100),
    avatar_url TEXT,
    bio TEXT,
    theme VARCHAR(20)
);

CREATE TABLE users.roles (
    user_id UUID REFERENCES users.main(id) ON DELETE CASCADE,
    feature VARCHAR(50) NOT NULL,
    role_level SMALLINT NOT NULL CHECK (role_level BETWEEN 2 AND 5),
    PRIMARY KEY (user_id, feature)
);

-- Optional: Index to quickly find who has access to a specific feature
CREATE INDEX idx_users_main_email ON users.main(email);
CREATE INDEX idx_users_roles_feature ON users.roles(feature);

-- Automatic updated_at trigger for both tables
CREATE OR REPLACE FUNCTION users.update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_main_updated_at
BEFORE UPDATE ON users.main
FOR EACH ROW EXECUTE FUNCTION users.update_updated_at_column();

CREATE TRIGGER update_users_detail_updated_at
BEFORE UPDATE ON users.detail
FOR EACH ROW EXECUTE FUNCTION users.update_updated_at_column();
