create or replace procedure client_set_active(
    p_client_id client.clients.id%type,
    p_active client.clients.active%type
)
language plpgsql
as $$
declare
    t_default_client_id client.clients.id%type;
begin
    t_default_client_id = client.client_default_id();

    if t_default_client_id = p_client_id then
        raise notice 'cannot set active status for default client';
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
            active = p_active
        where
            id = p_client_id
        ;
    end if;
end
$$;