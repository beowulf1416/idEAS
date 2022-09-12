create or replace function dimensions_fetch()
returns table (
    id common.dimensions.id%type,
    name common.dimensions.name%type
)
language plpgsql
as $$
begin
    return query
    select
        a.id,
        a.name
    from common.dimensions a
    ;
end
$$;