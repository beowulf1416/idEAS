create or replace function user_client_fetch(
    p_user_id iam.user_clients.user_id%type,
    p_active_only iam.user_clients.active%type
)
returns table (
    client_id client.clients.id%type,
    active client.clients.active%type,
    name client.clients.name%type,
    description client.clients.description%type,
    address client.clients.address%type,
    country_id client.clients.country_id%type,
    url client.clients.url%type
)
language plpgsql
as $$
begin
    if p_active_only then
        return query
        select
            c.id,
            c.active,
            c.name,
            c.description,
            c.address,
            c.country_id,
            c.url
        from iam.user_clients uc
            join client.clients c
                on uc.client_id = c.id
        where
            uc.user_id = p_user_id
            and uc.active = true
            and c.active = true
        ;
    else
        return query
        select
            c.id,
            c.active,
            c.name,
            c.description,
            c.address,
            c.country_id,
            c.url
        from iam.user_clients uc
            join client.clients c
                on uc.client_id = c.id
        where
            uc.user_id = p_user_id
        ;
    end if;
end
$$;