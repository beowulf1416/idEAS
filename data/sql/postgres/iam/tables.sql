/**
 * identity and access management
 */
create schema iam;
set schema 'iam';

\ir tables/permissions.sql

\ir tables/roles.sql
\ir tables/role_permissions.sql

\ir tables/users.sql
\ir tables/user_roles.sql
\ir tables/user_clients.sql

\ir tables/user_people.sql

-- \ir tables/user_registration.sql
\ir tables/user_invites.sql