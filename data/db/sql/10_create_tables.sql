/**
 * database creation script
 */

create database eas;

-- for gen_random_uuid()
create extension if not exists pgcrypto;

\ir common/create_tables.sql
\ir tenants/create_tables.sql
\ir iam/create_tables.sql
\ir accounting/create_tables.sql
\ir inventory/create_tables.sql