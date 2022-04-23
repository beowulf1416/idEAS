create or replace procedure assign_user_to_tenant (
    p_user_id iam.user_tenants.user_id%type,
    p_tenant_id iam.user_tenants.tenant_id%type
)
language plpgsql
as $$
begin
    insert into iam.user_tenants (
        user_id,
        tenant_id
    ) values (
        p_user_id,
        p_tenant_id
    );
end
$$;