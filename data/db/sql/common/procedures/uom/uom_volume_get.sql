create or replace function uom_volume_get (
    p_id common.uom_volume.id%type
)
returns table (
    id common.uom_volume.id%type,
    name common.uom_volume.name%type,
    symbol common.uom_volume.symbol%type,
    conversion common.uom_volume.conversion%type

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
    from common.uom_volume a
    where
        a.id = p_id
    ;
end
$$;