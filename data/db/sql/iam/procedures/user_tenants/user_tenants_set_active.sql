create or replace procedure user_tenants_set_active (
    p_user_id iam.user_tenants.user_id%type,
    p_tenant_id iam.user_tenants.tenant_id%type,
    p_active iam.user.tenants.active%type
)
language plpsql
as $$
begin
    update iam.user_tenants set
        active = p_active
    where
        user_id = p_user_id
        and tenant_id = p_tenant_id
    ;
end
$$;