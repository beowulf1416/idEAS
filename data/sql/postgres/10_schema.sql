create database eas;

create extension if not exists pgcrypto;

-- tables
\ir common/tables.sql
\ir domain/tables.sql
\ir iam/tables.sql

\ir crm/tables.sql


-- procs
\ir common/procs.sql
\ir domain/procs.sql
\ir iam/procs.sql

\ir crm/procs.sql