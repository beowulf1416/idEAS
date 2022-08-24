/**
 * initialize tenants schema
 */
set schema 'tenants';

create procedure init_schema()
language plpgsql
as $$
declare
    p_tenant_id uuid;
begin
    p_tenant_id := public.gen_random_uuid();

    call tenants.tenant_add(
        p_tenant_id,
        'default'
    );

    call tenants.tenant_set_active(
        p_tenant_id,
        true
    );
end
$$;

call init_schema();
drop procedure init_schema;