create or replace function currency_get (
    p_country_id common.currency.currency_alpha_3%type
)
return table (
    currency_alpha_3 common.currencies.currency_alpha_3%type,
    currency common.currencies.currency%type
)
language plpgsql
as $$
begin
    return query
    select
        a.currency_alpha_3,
        a.currency
    from common.currencies a
    where
        a.currency_alpha_3 = p_country_id
    ;
end
$$;