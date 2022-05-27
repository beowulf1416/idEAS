create or replace function uom_masses_get ()
returns table (
    id common.uom_masses.id%type,
    name common.uom_masses.name%type,
    symbol common.uom_masses.symbol%type,
    conversion common.uom_masses.conversion%type

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
    from common.uom_masses a
    ;
end
$$;