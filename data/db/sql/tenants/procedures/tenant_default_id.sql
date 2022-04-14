/**
 * return default tenant's id
 */
create or replace function tenant_default_id ()
returns tenants.tenants.id%type
language plpgsql
stable
leakproof
as $$
declare
    tenant_default_id tenants.tenants.id%type;
begin
    select
        id into tenant_default_id
    from tenants.tenants
    where
        name = 'default'
    ;

    return tenant_default_id;
end
$$;