set schema 'iam';

\ir procs/users/user_add.sql
\ir procs/users/user_set_password.sql
\ir procs/users/user_set_active.sql
\ir procs/users/user_authenticate.sql
\ir procs/users/user_fetch.sql
\ir procs/users/user_get.sql
\ir procs/users/user_get_by_email.sql
\ir procs/users/user_register.sql
\ir procs/users/user_register_complete.sql

\ir procs/roles/role_add.sql
\ir procs/roles/role_fetch.sql
\ir procs/roles/role_get.sql
\ir procs/roles/role_get_by_slug.sql
\ir procs/roles/role_set_active.sql
\ir procs/roles/role_update.sql