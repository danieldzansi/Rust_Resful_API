-- Add migration script here

CREATE TABLE todos (
    id UUID PRIMARY KEY ,
    description TEXT NOT NULL, 
    completed BOOLEAN NOT NULL DEFAULT FALSE ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)

