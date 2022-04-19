/**
 * initialize iam schema
 */
set schema 'iam';

create procedure init_schema()
language plpgsql
as $$
declare
    tenant_id uuid;
begin
    -- create permissions
    insert into iam.permissions (id, active, name, description) values 
    (1, true, 'tenant.users.view', 'view tenant users'),
    (2, true, 'tenant.users.add', 'add user to tenant');

    tenant_id := tenants.tenant_default_id();
end
$$;

call init_schema();
drop procedure init_schema;