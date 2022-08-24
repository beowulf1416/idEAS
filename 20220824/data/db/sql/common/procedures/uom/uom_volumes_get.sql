create or replace function uom_volumes_get ()
returns table (
    id common.uom_volumes.id%type,
    name common.uom_volumes.name%type,
    symbol common.uom_volumes.symbol%type,
    conversion common.uom_volumes.conversion%type

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
    from common.uom_volumes a
    ;
end
$$;