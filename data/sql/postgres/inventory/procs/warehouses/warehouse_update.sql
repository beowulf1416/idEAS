create or replace procedure warehouse_update(
    p_client_id inventory.warehouses.client_id%type,
    p_warehouse_id inventory.warehouses.id%type,
    p_name inventory.warehouses.name%type,
    p_description inventory.warehouses.description%type
)
language plpgsql
as $$
begin
    update inventory.warehouses set
        name = p_name,
        description = p_description
    where
        id = p_warehouse_id
        and client_id = p_client_id
    ;
end
$$;