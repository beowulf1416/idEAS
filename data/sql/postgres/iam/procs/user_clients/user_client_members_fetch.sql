create or replace function user_client_members_fetch(
    p_client_id iam.user_clients.client_id%type,
    p_active iam.user_clients.active%type
)
returns table (
    id iam.users.id%type,
    active iam.users.active%type,
    email iam.users.email%type
)
language plpgsql
as $$
begin
    return query
    select
        a.id,
        a.active,
        a.email
    from iam.users a
        inner join iam.user_clients b
            on a.id = b.user_id
    where
        b.client_id = p_client_id
        and b.active = p_active
    ;
end
$$;