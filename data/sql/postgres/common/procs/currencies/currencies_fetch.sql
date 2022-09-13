create or replace function currencies_fetch()
returns table (
    id common.currencies.currency%type,
    name common.currencies.unit_text%type,
    symbol common.currencies.symbol%type
)
language plpgsql
as $$
begin
    return query
    select
        a.currency,
        a.unit_text,
        a.symbol
    from common.currencies a
    ;
end
$$;