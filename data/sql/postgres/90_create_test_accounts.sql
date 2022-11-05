create procedure create_test_accounts()
language plpgsql
as $$
declare
    c_client_id client.clients.id%type := client.client_default_id();
    c_user_id iam.users.id%type := public.gen_random_uuid();
    c_permission_client_add iam.permissions.id%type := iam.permission_get_id('client.add');
    c_admin_role_id iam.roles.id%type;
begin
    c_admin_role_id := iam.role_get_id(c_client_id, 'administrator');

    -- add administrator account
    call iam.user_add(
        c_user_id,
        'administrator@localhost.com'
    );

    call iam.user_set_password(
        c_user_id,
        'TestTest88**'
    );

    call iam.user_set_active(
        c_user_id,
        true
    );

    call iam.user_role_add(
        c_user_id,
        c_admin_role_id
    );

    call iam.user_role_set_active(
        c_user_id,
        c_admin_role_id,
        true
    );
end
$$;

call create_test_accounts();
drop procedure create_test_accounts;