create or replace procedure assign_user_to_tenant (
    p_user_id iam.user_tenants.user_id%type,
    p_tenant_id iam.user_tenants.tenant_id%type
)
language plpgsql
as $$
declare
    t_role_everybody_id iam.roles.id%type;
begin
    insert into iam.user_tenants (
        user_id,
        tenant_id
    ) values (
        p_user_id,
        p_tenant_id
    );

    -- add user to 'everybody' role
    select
        r.id into t_role_everybody_id
    from iam.roles r
    where
        r.tenant_id = p_tenant_id
        and r.name = 'everybody'
    ;

    insert into iam.user_roles (
        user_id,
        role_id
    ) values (
        p_user_id,
        t_role_everybody_id
    );
end
$$;