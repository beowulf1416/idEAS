create or replace procedure location_add(
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
    insert into inventory.locations (
        id,
        client_id,
        aisle,
        shelf,
        bin,
        pallet,
        level,
        floor
    ) values (
        p_location_id,
        p_client_id,
        p_aisle,
        p_shelf,
        p_bin,
        p_pallet,
        p_level,
        p_floor
    );
end
$$;