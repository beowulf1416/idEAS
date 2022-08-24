create or replace function currencies_get (
    p_filter varchar(100) default '',
    p_items int default null,
    p_page int default null
)
return table (
    currency_alpha_3 common.currencies.currency_alpha_3%type,
    currency common.currencies.currency%type
)
language plpgsql
as $$
begin
    if p_filter = '' then
        begin
            return query
            select
                a.currency_alpha_3,
                a.currency
            from common.currencies a
            limit p_items
            offset p_page * p_items
            ;
        end
    else
        begin
            return query
            select
                a.currency_alpha_3,
                a.currency
            from common.currencies a
            where
                a.concat(
                    a.currency_alpha_3,
                    a.currency
                ) like p_filter
            limit p_items
            offset p_page * p_items
            ;
        end
end
$$;