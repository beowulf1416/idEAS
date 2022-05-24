set schema 'iam';


\ir procedures/users/user_add.sql
\ir procedures/users/user_set_active.sql
\ir procedures/users/user_set_password.sql
\ir procedures/users/user_authenticate.sql
\ir procedures/users/user_get_by_email.sql
\ir procedures/users/user_get_id.sql

\ir procedures/roles/role_add.sql
\ir procedures/roles/role_set_active.sql
\ir procedures/roles/roles_get.sql

\ir procedures/permissions/permissions_get.sql

\ir procedures/user_tenants/assign_user_to_tenant.sql
\ir procedures/user_tenants/revoke_user_from_tenant.sql
\ir procedures/user_tenants/user_tenants_get.sql
\ir procedures/user_tenants/user_tenants_set_active.sql

\ir procedures/user_roles/assign_user_to_role.sql

\ir procedures/role_permissions/assign_permission_to_role.sql
\ir procedures/role_permissions/revoke_permission_from_role.sql
\ir procedures/role_permissions/role_permission_set_active.sql