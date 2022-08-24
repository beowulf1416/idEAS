create or replace function uom_mass_get (
    p_id common.uom_mass.id%type
)
returns table (
    id common.uom_mass.id%type,
    name common.uom_mass.name%type,
    symbol common.uom_mass.symbol%type,
    conversion common.uom_mass.conversion%type

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
    from common.uom_mass a
    where
        a.id = p_id
    ;
end
$$;