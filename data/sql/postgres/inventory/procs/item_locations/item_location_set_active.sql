create or replace procedure item_location_set_active(
    p_id inventory.item_locations.id%type,
    p_active inventory.item_locations.active%type
)
language plpgsql
as $$
begin
    update inventory.item_locations set
        active = p_active
    where
        id = p_id
    ;
end
$$;