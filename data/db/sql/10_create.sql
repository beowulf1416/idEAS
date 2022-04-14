/**
 * database creation script
 */

create database saas;

-- for gen_random_uuid()
create extension if not exists pgcrypto;

\ir common/create.sql
\ir tenants/create.sql
\ir iam/create.sql
