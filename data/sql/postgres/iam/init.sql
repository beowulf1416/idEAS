set schema 'iam';

create procedure initialize()
language plpgsql
as $$
declare
    c_client_id client.clients.id%type := client.client_default_id();
    c_permission_user_current_id iam.permissions.id%type := iam.permission_get_id('user.current');
    c_permission_user_profile_id iam.permissions.id%type := iam.permission_get_id('user.profile');

    c_permission_client_add iam.permissions.id%type := iam.permission_get_id('client.add');
    c_permission_client_update iam.permissions.id%type := iam.permission_get_id('client.update');
    c_permission_client_update_any iam.permissions.id%type := iam.permission_get_id('client.update.any');
    c_permission_client_active iam.permissions.id%type := iam.permission_get_id('client.active');
    c_permission_client_view iam.permissions.id%type := iam.permission_get_id('client.view');
    c_permission_client_view_any iam.permissions.id%type := iam.permission_get_id('client.view.any');

    c_role_id iam.roles.id%type:= public.gen_random_uuid();
    c_admin_role_id iam.roles.id%type := public.gen_random_uuid();
begin
    -- add default role to default client
    call iam.role_add(
        c_role_id,
        c_client_id,
        'default',
        'default'
    );

    -- set role to active
    call iam.role_set_active(
        c_role_id,
        true
    );

    -- add permissions to role //TODO add required permissions
    insert into iam.role_permissions (active, role_id, permission_id) values
    (true, c_role_id, c_permission_user_current_id),
    (true, c_role_id, c_permission_user_profile_id)
    ;

    -- add administrator role
    call iam.role_add(
        c_admin_role_id,
        c_client_id,
        'administrator',
        'administrator'
    );

    call iam.role_set_active(
        c_admin_role_id,
        true
    );

    insert into iam.role_permissions (active, role_id, permission_id) values
    (true, c_admin_role_id, c_permission_user_current_id),
    (true, c_admin_role_id, c_permission_user_profile_id),
    (true, c_admin_role_id, c_permission_client_add),
    (true, c_admin_role_id, c_permission_client_update),
    (true, c_admin_role_id, c_permission_client_update_any),
    (true, c_admin_role_id, c_permission_client_active),
    (true, c_admin_role_id, c_permission_client_view),
    (true, c_admin_role_id, c_permission_client_view_any)
    ;
end
$$;

call initialize();
drop procedure initialize;