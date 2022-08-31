create or replace procedure client_add(
    p_client_id client.clients.id%type,
    p_name client.clients.name%type,
    p_description client.clients.description%type,
    p_address client.clients.address%type,
    p_country_id client.clients.country_id%type,
    p_url client.clients.url%type
)
language plsql
as $$
begin
    insert into clients (
        id,
        active,
        name,
        description,
        address,
        country_id,
        url
    ) values (
        p_client_id,
        false,
        p_description,
        p_address,
        p_country_id,
        p_url
    );
end
$$;