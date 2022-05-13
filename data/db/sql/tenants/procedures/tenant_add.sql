create or replace procedure tenant_add (
    p_tenant_id tenants.tenants.id%type,
    p_tenant_name tenants.tenants.name%type
)
language plpgsql
as $$
declare
    t_role_client_admin_id iam.roles.id%type;
    t_role_everybody_id iam.roles.id%type;

    t_permission_id iam.permissions.id%type;
    t_permission_user_current iam.permissions.id%type;
begin
    insert into tenants.tenants (
        id,
        name
    ) values (
        p_tenant_id,
        p_tenant_name
    );

    select
        p.id into t_permission_id 
    from iam.permissions p
    where
        p.name = 'dashboard.view'
    ;

    select
        p.id into t_permission_user_current 
    from iam.permissions p
    where
        p.name = 'user.current'
    ;

    -- create default roles
    t_role_client_admin_id := public.gen_random_uuid();
    t_role_everybody_id := public.gen_random_uuid();

    if (select to_regproc('iam.role_add')) is null then
        begin
            -- client administrator role
            insert into iam.roles (
                id,
                tenant_id,
                name,
                description
            ) values (
                t_role_client_admin_id,
                p_tenant_id,
                'client administrator',
                'client administrator'
            );

            update iam.roles set
                active = true
            where
                id = t_role_client_admin_id
            ;

            --everybody role
            insert into iam.roles (
                id,
                tenant_id,
                name,
                description
            ) values (
                t_role_everybody_id,
                p_tenant_id,
                'everybody',
                'everybody'
            );

            update iam.roles set
                active = true
            where
                id = t_role_everybody_id
            ;

            -- assign 'user.current' permission to 'everybody' role
            insert into iam.role_permissions (
                role_id,
                permission_id
            ) values (
                t_role_everybody_id,
                t_permission_user_current
            );

            update iam.role_permissions set
                active = true
            where
                role_id = t_role_everybody_id
                and permission_id = t_permission_user_current
            ;
        end;
    else
        begin
            -- client administrator role
            call iam.role_add(
                t_role_client_admin_id,
                p_tenant_id,
                'client administrator',
                'client administrator'
            );

            call iam.role_set_active(
                t_role_client_admin_id,
                true
            );

            -- everybody role
            call iam.role_add(
                t_role_everybody_id,
                p_tenant_id,
                'everybody',
                'everybody'
            );

            call iam.role_set_active(
                t_role_everybody_id,
                true
            );

            -- assign 'user.current' permission to 'everybody' role
            call iam.assign_permission_to_role(
                t_role_everybody_id,
                t_permission_user_current
            );

            call iam.role_permission_set_active (
                t_role_everybody_id,
                t_permission_user_current,
                true
            );
        end;
    end if;
end
$$;