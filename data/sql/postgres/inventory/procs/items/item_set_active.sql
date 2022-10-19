create or replace procedure item_set_active(
    p_item_id inventory.items.id%type,
    p_active inventory.items.active%type
)
language plpgsql
as $$
begin
    update inventory.items set
        active = p_active
    where
        id = p_item_id
    ;
end
$$