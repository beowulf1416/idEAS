create or replace function user_client_fetch(
    p_user_id iam.user_clients.user_id%type,
    p_active_only iam.user_clients.active%type
)
returns table (
    client_id client.clients.id%type,
    active client.clients.active%type,
    name client.clients.name%type
)
language plpgsql
as $$
begin
    if p_active_only then
        return query
        select
            c.id,
            c.active,
            c.name
        from iam.user_clients uc
            join client.clients c
                on uc.client_id = c.id
        where
            uc.active = p_active
        ;
    else
        return query
        select
            c.id,
            c.active,
            c.name
        from iam.user_clients uc
            join client.clients c
                on uc.client_id = c.id
        ;
    end if;
end
$$;