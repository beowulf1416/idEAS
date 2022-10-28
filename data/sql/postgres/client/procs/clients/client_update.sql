create or replace procedure client_update(
    p_client_id client.clients.id%type,
    p_name client.clients.name%type,
    p_active client.clients.active%type,
    p_description client.clients.description%type,
    p_address client.clients.address%type,
    p_country_id client.clients.country_id%type,
    p_url client.clients.url%type
)
language plpgsql
as $$
declare
    t_default_client_id client.clients.id%type;
begin
    t_default_client_id = client.client_default_id();

    if t_default_client_id = p_client_id then
        raise notice 'cannot update default client';
    else
        insert into client.clients_scd (
            client_id,
            active,
            name,
            description,
            address,
            country_id,
            url
        )
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
            id = p_client_id
        ;

        update client.clients set
            name = p_name,
            active = p_active,
            address = p_address,
            description = p_description,
            country_id = p_country_id,
            url = p_url
        where
            id = p_client_id
        ;
    end if;
end
$$;