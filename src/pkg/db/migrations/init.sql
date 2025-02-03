-- Create the users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    pic_obj_name TEXT
);

-- Create the groups table
CREATE TABLE groups (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    member TEXT NOT NULL
);

-- Create the chats table
CREATE TABLE chats (
    id SERIAL PRIMARY KEY,
    "from" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    "to" UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    body BYTEA NOT NULL
);

-- Indexes for optimization (optional)
CREATE INDEX idx_chats_from ON chats ("from");
CREATE INDEX idx_chats_to ON chats ("to");
