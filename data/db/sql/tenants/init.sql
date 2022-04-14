/**
 * initialize tenants schema
 */
set schema 'tenants';

create procedure init_schema()
language plpgsql
as $$
declare
    tenant_id uuid;
begin
    tenant_id := public.gen_random_uuid();

    call tenants.tenant_add(
        tenant_id,
        'default'
    );

    call tenants.tenant_set_active(
        tenant_id,
        true
    );
end
$$;

call init_schema();
drop procedure init_schema;