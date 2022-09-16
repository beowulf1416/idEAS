create or replace procedure organization_set_active(
    p_organization_id client.organizations.id%type,
    p_active client.organizations.active%type
)
language plpgsql
as $$
begin
    insert into client.organizations (
        org_id,
        active,
        name,
        description
    )
    select
        a.id,
        a.active,
        a.name,
        a.description
    from client.organizations_scd a
    where
        id = p_organization_id
    ;
    
    update client.organizations set
        active = p_active
    where
        id = p_organization_id
    ;
end
$$;