create or replace procedure user_add(
    p_id iam.users.id%type,
    p_email iam.users.email%type
)
language plpgsql
as $$
declare
    t_salt text;
    t_pw text;
    t_default_client_id iam.user_clients.client_id%type;
begin
    -- generate random password
    t_salt := public.gen_salt('md5');
    t_pw := public.crypt(random()::text, t_salt);

    insert into iam.users (
        id,
        email,
        pw
    ) values (
        p_id,
        p_email,
        t_pw
    );

    -- add user to default client
    t_default_client_id = client.client_default_id();
    call iam.user_client_add(
        p_id,
        t_default_client_id
    );

    -- set active
    call iam.user_client_set_active(
        p_id,
        t_default_client_id,
        true
    );

    -- associate record with user
    call iam.user_people_update(
        p_id,
        '',
        '',
        '',
        '',
        ''
    );
end
$$;