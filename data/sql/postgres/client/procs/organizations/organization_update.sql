create or replace procedure organization_update(
    p_client_id client.clients.id%type,
    p_organization_id client.organizations.id%type,
    p_name client.organizations.name%type,
    p_description client.organizations.description%type
)
language plpgsql
as $$
begin
    update client.organizations set
        name = p_name,
        description = p_description
    where
        id = p_organization_id
    ;
end
$$;