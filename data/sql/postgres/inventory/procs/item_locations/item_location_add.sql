create or replace procedure item_location_add(
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
declare
    t_perishable inventory.items.perishable%type;
begin
    -- check if item is perishable
    if exists(
        select
            *
        from inventory.items a
        where
            a.id = p_item_id
            and a.perishable = true
    ) and p_expiry is null then
        raise exception 'expiry date required' using errcode = 'data_exception';
    end if;

    insert into inventory.item_locations (
        id,
        client_id,
        item_id,
        location_id,
        batch_id,
        lot_id,
        expiry,
        balance,
        uom_id
    ) values (
        p_id,
        p_client_id,
        p_item_id,
        p_location_id,
        p_batch_id,
        p_lot_id,
        p_expiry,
        p_balance,
        p_uom_id
    );
end
$$;