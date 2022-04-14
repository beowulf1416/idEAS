/**
 * identity and access management
 */
create schema iam;
set schema 'iam';

\ir tables/permissions.sql
\ir tables/roles.sql
\ir tables/users.sql


\ir procedures/users/user_add.sql
\ir procedures/users/user_set_active.sql
\ir procedures/users/user_authenticate.sql
\ir procedures/users/user_get_by_email.sql
\ir procedures/users/user_get_id.sql