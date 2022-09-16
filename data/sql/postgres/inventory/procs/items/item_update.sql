create or replace procedure item_update(
    p_client_id inventory.items.client_id%type,
    p_item_id inventory.items.id%type,
    p_name inventory.items.name%type,
    p_description inventory.items.description%type,
    
    p_sku inventory.items.sku%type,
    p_upc inventory.items.upc%type,
    
    p_dimension_id inventory.items.dimension_id%type,
    p_uom_id inventory.items.uom_id%type,
    
    p_volume inventory.items.volume%type,
    p_weight inventory.items.weight%type,
    p_shelf_width inventory.items.shelf_width%type,
    p_shelf_height inventory.items.shelf_height%type,
    p_shelf_depth inventory.items.shelf_depth%type,

    p_perishable inventory.items.perishable%type,
    p_stocked inventory.items.stocked%type,
    p_purchased inventory.items.purchased%type,
    p_sold inventory.items.sold%type,
    p_manufactured inventory.items.manufactured%type
)
language plpgsql
as $$
begin
    update inventory.items set
        name = p_name,
        description = p_description,
        sku = p_sku,
        upc = p_upc,
        dimension_id = p_dimension_id,
        uom_id = p_uom_id,
        volume = p_volume,
        weight = p_weight,
        shelf_width = p_shelf_width,
        shelf_height = p_shelf_height,
        shelf_depth = p_shelf_depth,
        perishable = p_perishable,
        stocked = p_stocked,
        purchased = p_purchased,
        sold = p_sold,
        manufactured = p_manufactured
    where
        id = p_item_id
        and client_id = p_client_id
    ;
end
$$;