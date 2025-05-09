-- This file is a log of scripts which must be executed manually outside the migration scheme.

-- Refinery cannot execute CREATE DATABASE commands because migrations are executed within transactions.
-- Postgres does not allow CREATE DATABASE to execute within a transaction context.
create database authelia;
