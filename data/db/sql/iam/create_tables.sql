/**
 * identity and access management
 */
create schema iam;
set schema 'iam';

\ir tables/permissions.sql
\ir tables/roles.sql
\ir tables/users.sql