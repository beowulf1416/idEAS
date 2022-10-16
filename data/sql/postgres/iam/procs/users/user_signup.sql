create or replace procedure user_signup(
    p_user_id iam.users.id%type,
    p_email iam.users.email%type,
    p_password iam.users.pw%type
)
language plpgsql
as $$
declare
    t_client_id client.clients.id%type;
    t_role_id iam.roles.id%type;
begin
    insert into iam.users (
        id,
        active,
        email,
        pw
    ) values (
        p_user_id,
        true,
        p_email,
        p_password
    );

    -- add user to default client
    t_client_id := client.client_default_id();
    call iam.user_client_add(
        p_user_id,
        t_client_id
    );

    -- add user to default role
    select
        a.id into t_role_id
    from iam.roles a
    where
        a.client_id = t_client_id
        and a.name = 'default'
    ;

    call iam.user_role_add(
        p_user_id,
        t_role_id
    );
end
$$;