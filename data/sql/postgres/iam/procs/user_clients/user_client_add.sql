create or replace procedure user_client_add(
    p_user_id iam.user_clients.user_id%type,
    p_client_id iam.user_clients.client_id%type
)
language plpgsql
as $$
begin
    insert into iam.user_clients (
        user_id,
        client_id
    ) values (
        p_user_id,
        p_client_id
    )
    on conflict (user_id, client_id)
    do nothing;
end
$$;