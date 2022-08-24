create or replace procedure revoke_user_from_tenant (
    p_user_id iam.user_tenants.user_id%type,
    p_tenant_id iam.user_tenants.tenant_id%type
)
language plpgsql
as $$
begin
    delete from iam.user_tenants
    where
        user_id = p_user_id
        and tenant_id = p_tenant_id
    ;
end
$$;
