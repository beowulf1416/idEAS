create or replace procedure user_client_invite(
    p_client_id iam.user_clients.client_id%type,
    p_email iam.users.email%type
)
language plpgsql
as $$
declare
    t_user_id iam.users.id%type;
begin
    t_user_id := public.gen_random_uuid();
    call iam.user_add(
        t_user_id,
        p_email
    );

    call iam.user_client_add(
        t_user_id,
        p_client_id
    );
end
$$;

comment on procedure user_client_invite is 'invite a user join client';