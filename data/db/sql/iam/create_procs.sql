set schema 'iam';


\ir procedures/users/user_add.sql
\ir procedures/users/user_set_active.sql
\ir procedures/users/user_authenticate.sql
\ir procedures/users/user_get_by_email.sql
\ir procedures/users/user_get_id.sql

\ir procedures/roles/role_add.sql
\ir procedures/roles/role_set_active.sql

\ir procedures/user_tenants/assign_user_to_tenant.sql

\ir procedures/user_roles/assign_user_to_role.sql