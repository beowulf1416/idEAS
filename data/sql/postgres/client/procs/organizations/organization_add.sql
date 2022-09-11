create or replace procedure organization_add(
    p_client_id client.organizations.client_id%type,
    p_organization_id client.organizations.id%type,
    p_name client.organizations.name%type,
    p_description client.organizations.description%type
)
language plpgsql
as $$
begin
    insert into client.organizations (
        id,
        client_id,
        active,
        name,
        description
    ) values (
        p_organization_id,
        p_client_id,
        false,
        p_name,
        p_description
    );
end
$$;