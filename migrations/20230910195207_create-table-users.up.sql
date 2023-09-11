-- Add up migration script here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS
    "users" (
        id UUID NOT NULL PRIMARY KEY DEFAULT (uuid_generate_v4()),
        name VARCHAR(100) NOT NULL,
        email VARCHAR(255) NOT NULL UNIQUE,
        password VARCHAR(100) NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
        updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );