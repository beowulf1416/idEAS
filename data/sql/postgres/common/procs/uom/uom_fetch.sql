create or replace function uom_fetch(
    p_dimension_id common.uom.dimension_id%type
)
returns table (
    id common.uom.id%type,
    name common.uom.name%type,
    description common.uom.description%type,
    symbol common.uom.symbol%type
)
language plpgsql
as $$
begin
    return query
    select
        a.id,
        a.name,
        a.description,
        a.symbol
    from common.uom a
    where
        a.dimension_id = p_dimension_id
    ;
end
$$;