create or replace procedure tenant_add (
    p_tenant_id tenants.tenants.id%type,
    p_tenant_name tenants.tenants.name%type
)
language plpgsql
as $$
declare
    t_role_id iam.roles.id%type;
begin
    insert into tenants.tenants (
        id,
        name
    ) values (
        p_tenant_id,
        p_tenant_name
    );

    -- create default role
    t_role_id := public.gen_random_uuid();
    if (select to_regproc('iam.role_add')) is null then
        begin
            insert into iam.roles (
                id,
                tenant_id,
                name,
                description
            ) values (
                t_role_id,
                p_tenant_id,
                'client administrator',
                'client administrator'
            );

            update iam.roles set
                active = true
            where
                id = t_role_id
            ;
        end;
    else
        begin
            call iam.role_add(
                t_role_id,
                p_tenant_id,
                'client administrator',
                'client administrator'
            );

            call iam.role_set_active(
                t_role_id,
                true
            );
        end;
    end if;
end
$$;