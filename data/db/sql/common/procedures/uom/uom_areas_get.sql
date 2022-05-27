create or replace function uom_areas_get ()
returns table (
    id common.uom_areas.id%type,
    name common.uom_areas.name%type,
    symbol common.uom_areas.symbol%type,
    conversion common.uom_areas.conversion%type

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
    from common.uom_areas a
    ;
end
$$;