create or replace procedure client_add(
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
begin
    insert into client.clients (
        id,
        active,
        name,
        description,
        address,
        country_id,
        url
    ) values (
        p_client_id,
        p_active,
        p_name,
        p_description,
        p_address,
        p_country_id,
        p_url
    );
end
$$;