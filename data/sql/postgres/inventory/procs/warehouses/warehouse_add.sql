create or replace procedure warehouse_add(
    p_client_id inventory.warehouses.client_id%type,
    p_warehouse_id inventory.warehouses.id%type,
    p_name inventory.warehouses.name%type,
    p_description inventory.warehouses.description%type
)
language plpgsql
as $$
begin
    insert into inventory.warehouses (
        id,
        client_id,
        name,
        description
    ) values (
        p_warehouse_id,
        p_client_id,
        p_name,
        p_description
    );
end
$$;