create or replace procedure location_update(
    p_client_id inventory.locations.client_id%type,
    p_location_id inventory.locations.id%type,
    p_aisle inventory.locations.aisle%type,
    p_shelf inventory.locations.shelf%type,
    p_bin inventory.locations.bin%type,
    p_pallet inventory.locations.pallet%type,
    p_level inventory.locations.level%type,
    p_floor inventory.locations.floor%type
)
language plpgsql
as $$
begin
    update inventory.locations set
        aisle = p_aisle,
        shelf = p_shelf,
        bin = p_bin,
        pallet = p_pallet,
        level = p_level,
        floor = p_floor
    where
        id = p_location_id
        and client_id = p_client_id
    ;
end
$$;