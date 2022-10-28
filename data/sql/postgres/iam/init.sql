set schema 'iam';

create procedure initialize()
language plpgsql
as $$
declare
    c_client_id client.clients.id%type := client.client_default_id();
    c_role_id iam.roles.id%type := public.gen_random_uuid();
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
    (true, c_role_id, 1),
    (true, c_role_id, 2)
    ;
end
$$;

call initialize();
drop procedure initialize;