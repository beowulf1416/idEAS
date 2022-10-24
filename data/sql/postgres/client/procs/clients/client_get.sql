create or replace function client_get(
    p_client_id client.clients.id%type
)
returns table (
    id client.clients.id%type,
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
    return query
    select
        a.id,
        a.active,
        a.name,
        a.description,
        a.address,
        a.country_id,
        a.url
    from client.clients a
    where
        a.id = p_client_id
    ;
end
$$;