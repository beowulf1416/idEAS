create or replace function client_fetch(
    p_filter varchar(100),
    p_active boolean default true,
    p_items int default 10,
    p_page int default 0
)
returns table (
    id client.clients.id%type,
    active client.clients.id%type,
    name client.clients.name%type,
    description client.clients.description%type,
    address client.clients.address%type,
    country_id client.clients.country_id%type,
    url client.clients.url%type
)
language plpgsql
as $$
begin
    if p_filter = '' then
        if p_active = true then
            return query
            select
                id,
                active,
                name,
                description,
                address,
                country_id,
                url
            from client.clients
            where
                active = true
            ;
        else
            return query
            select
                id,
                active,
                name,
                description,
                address,
                country_id,
                url
            from client.clients
            ;
        end if;
    else
        if p_active = true then
            return query
            select
                id,
                active,
                name,
                description,
                address,
                country_id,
                url
            from client.clients
            where
                active = true
                and name like p_filter
            ;
        else
            return query
            select
                id,
                active,
                name,
                description,
                address,
                country_id,
                url
            from client.clients
                name like p_filter
            ;
        end if;
    end if;
end
$$;