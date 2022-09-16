create or replace procedure location_set_active(
    p_location_id inventory.locations.id%type,
    p_active inventory.locations.active%type
)
language plpgsql
as $$
begin
    update inventory.locations set
        active = p_active
    where
        id = p_location_id
    ;
end
$$;