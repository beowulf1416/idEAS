create or replace function uom_quantities_get ()
returns table (
    id common.uom_quantities.id%type,
    name common.uom_quantities.name%type,
    symbol common.uom_quantities.symbol%type,
    conversion common.uom_quantities.conversion%type

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
    from common.uom_quantities a
    ;
end
$$;