create database eas;

create extension if not exists pgcrypto;

-- tables
\ir common/tables.sql
\ir client/tables.sql
\ir crm/tables.sql
\ir iam/tables.sql
\ir inventory/tables.sql


-- procs
\ir common/procs.sql
\ir client/procs.sql
\ir crm/procs.sql
\ir iam/procs.sql
\ir inventory/procs.sql