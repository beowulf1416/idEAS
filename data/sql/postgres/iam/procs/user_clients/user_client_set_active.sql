create or replace procedure user_client_set_active(
    p_user_id iam.user_clients.user_id%type,
    p_client_id iam.user_clients.client_id%type,
    p_active iam.user_clients.active%type
)
language plpgsql
as $$
begin
    update iam.user_clients set
        active = p_active
    where
        user_id = p_user_id
        and client_id = p_client_id
    ;
end
$$;