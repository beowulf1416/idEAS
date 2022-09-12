create or replace procedure organization_set_active(
    p_organization_id client.organizations.id%type,
    p_active client.organizations.active%type
)
language plpgsql
as $$
begin
    update client.organizations set
        active = p_active
    where
        id = p_organization_id
    ;
end
$$;