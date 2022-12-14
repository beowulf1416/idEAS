create or replace function warehouse_fetch(
    p_client_id inventory.warehouses.client_id%type
)
returns table (
    id inventory.warehouses.id%type,
    active inventory.warehouses.active%type,
    created inventory.warehouses.created%type,
    name inventory.warehouses.name%type,
    description inventory.warehouses.description%type
)
language plpgsql
as $$
begin
    return query
    select
        a.id,
        a.active,
        a.created,
        a.name,
        a.description
    from inventory.warehouses a
    where
        a.client_id = p_client_id
    ;
end
$$;