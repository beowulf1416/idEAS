create or replace function uom_area_get (
    p_id common.uom_volume.id%type
)
returns table (
    id common.uom_area.id%type,
    name common.uom_area.name%type,
    symbol common.uom_area.symbol%type,
    conversion common.uom_area.conversion%type

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
    from common.uom_area a
    where
        a.id = p_id
    ;
end
$$;