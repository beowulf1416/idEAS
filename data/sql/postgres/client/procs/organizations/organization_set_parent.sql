create or replace procedure organization_set_parent(
    p_client_id client.organization_tree.client_id%type,
    p_org_id client.organization_tree.org_id%type,
    p_parent_org_id client.organization_tree.parent_org_id%type
)
language plpgsql
as $$
begin
    insert into client.organization_tree (
        client_id,
        org_id,
        parent_org_id
    ) values (
        p_client_id,
        p_org_id,
        p_parent_org_id
    );
end
$$;