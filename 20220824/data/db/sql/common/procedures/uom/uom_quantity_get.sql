create or replace function uom_quantity_get (
    p_id common.uom_quantity.id%type
)
returns table (
    id common.uom_quantity.id%type,
    name common.uom_quantity.name%type,
    symbol common.uom_quantity.symbol%type,
    conversion common.uom_quantity.conversion%type

)
language plpgsql
as $$
begin
    return query
    select
        a.id,
        a.name,
        a.symbol,
        a.conversion
    from common.uom_quantity a
    where
        a.id = p_id
    ;
end
$$;