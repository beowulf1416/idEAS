/**
* create test accounts for testing
 */
create procedure init_schema()
language plpgsql
as $$
declare
    t_tenant_id tenants.tenants.id%type;
    t_role_id iam.roles.id%type;
    t_user_id iam.users.id%type;
begin
    t_tenant_id = public.gen_random_uuid();
    t_role_id = public.gen_random_uuid();
    t_user_id = public.gen_random_uuid();

    -- create test tenant and make active
    call tenants.tenant_add(
        t_tenant_id,
        'test tenant'
    );

    call tenants.tenant_set_active(
        t_tenant_id,    
        true
    );


    -- create default role for tenant
    call iam.role_add(
        t_role_id,
        t_tenant_id,
        'default',
        'default role'
    );

    call iam.role_set_active(
        t_role_id,
        true
    );

    -- assign permissions to role
    call iam.assign_permission_to_role(
        t_role_id,
        1 -- user.current permission
    );

    call iam.assign_permission_to_role(
        t_role_id,
        2 -- user.profile permission
    );


    call iam.assign_permission_to_role(
        t_role_id,
        3 -- dashboard.view permission
    );


    -- create user and assign to tenant
    call iam.user_add(
        t_user_id,
        'testemail@email.com',
        'thisIs1Password'
    );

    call iam.user_set_active(
        t_user_id,
        true
    );

    call iam.assign_user_to_tenant(
        t_user_id,
        t_tenant_id
    );

    call iam.assign_user_to_role(
        t_user_id,
        t_role_id
    );
end
$$;

call init_schema();
drop procedure init_schema();