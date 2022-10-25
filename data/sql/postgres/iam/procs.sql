set schema 'iam';

\ir procs/users/user_add.sql
\ir procs/users/user_set_password.sql
\ir procs/users/user_set_active.sql
\ir procs/users/user_authenticate.sql
\ir procs/users/user_fetch.sql
\ir procs/users/user_get.sql
\ir procs/users/user_get_by_email.sql

\ir procs/users/user_permissions_fetch.sql

-- \ir procs/users/user_register.sql
-- \ir procs/users/user_register_complete.sql
-- \ir procs/users/user_register_get.sql

\ir procs/users/user_signup.sql

\ir procs/user_clients/user_client_add.sql
\ir procs/user_clients/user_client_set_active.sql
\ir procs/user_clients/user_client_fetch.sql

\ir procs/roles/role_add.sql
\ir procs/roles/role_fetch.sql
\ir procs/roles/role_get.sql
\ir procs/roles/role_set_active.sql
\ir procs/roles/role_update.sql

\ir procs/role_permissions/role_permission_add.sql
\ir procs/role_permissions/role_permission_set_active.sql

\ir procs/user_roles/user_role_add.sql
\ir procs/user_roles/user_role_set_active.sql