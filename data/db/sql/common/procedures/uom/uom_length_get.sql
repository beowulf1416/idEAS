create or replace function uom_length_get (
    p_id common.uom_length.id%type
)
returns table (
    id common.uom_length.id%type,
    name common.uom_length.name%type,
    symbol common.uom_length.symbol%type,
    conversion common.uom_length.conversion%type

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
    from common.uom_length a
    where
        a.id = p_id
    ;
end
$$;