create or replace function permissions_get (
    p_user_id iam.users.id%type,
    p_tenant_id iam.user_tenants.tenant_id%type
)
returns table (
    permission_id iam.permissions.id%type,
    permission_name iam.permissions.name%type
)
language plpgsql
as $$
begin
    return query
    select
        distinct
        p.id,
        p.name
    from iam.user_roles ur
        join iam.role_permissions rp
            on ur.role_id = rp.role_id
        join iam.roles r
            on rp.role_id = r.id
                and r.active = true
        join iam.permissions p
            on rp.permission_id = p.id
                and p.active = true
    where
        ur.user_id = p_user_id
    ;
end
$$;