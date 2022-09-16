create or replace procedure warehouse_set_active(
    p_warehouse_id inventory.warehouses.id%type,
    p_active inventory.warehouses.active%type
)
language plpgsql
as $$
begin
    update inventory.warehouses set
        active = p_active
    where
        id = p_warehouse_id
    ;
end
$$;