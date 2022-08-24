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
    tenant_id := tenants.tenant_default_id();
end
$$;

call init_schema();
drop procedure init_schema;