create or replace function user_tenants_get (
    p_user_id iam.user_tenants.user_id%type
)
returns table (
    tenant_id iam.user_tenants.tenant_id%type,
    tenant_name tenants.tenants.name%type
)
language plpgsql
as $$
begin
    return query
    select
        t.id,
        t.name
    from iam.user_tenants ut
        join tenants.tenants t
            on ut.tenant_id = t.id
                and t.active = true
    where
        ut.user_id = p_user_id
    ;
end
$$;
