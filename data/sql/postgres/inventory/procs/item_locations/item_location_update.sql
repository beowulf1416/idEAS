create or replace procedure item_location_update(
    p_client_id inventory.item_locations.client_id%type,
    p_item_id inventory.item_locations.item_id%type,
    p_location_id inventory.item_locations.location_id%type,
    p_id inventory.item_locations.id%type,
    p_batch_id inventory.item_locations.batch_id%type,
    p_lot_id inventory.item_locations.lot_id%type,
    p_expiry inventory.item_locations.expiry%type,
    p_balance inventory.item_locations.balance%type,
    p_uom_id inventory.item_locations.uom_id%type
)
language plpgsql
as $$
begin
    update inventory.item_locations set
        item_id = p_item_id,
        location_id = p_location_id,
        batch_id = p_batch_id,
        lot_id = p_lot_id,
        expiry = p_expiry,
        balance = p_balance,
        uom_id = p_uom_id
    where
        id = p_id
        and client_id = p_client_id
    ;
end
$$;