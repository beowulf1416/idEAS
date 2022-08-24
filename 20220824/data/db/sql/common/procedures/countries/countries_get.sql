create or replace function countries_get (
    p_filter varchar(100) default '',
    p_items int default null,
    p_page int default null
)
returns table (
    iso3166_1_numeric common.countries.iso3166_1_numeric%type,
    official_name_en common.countries.official_name_en%type,
    iso3166_1_alpha_2 common.countries.iso3166_1_alpha_2%type,
    iso3166_1_alpha_3 common.countries.iso3166_1_alpha_3%type
)
language plpgsql
as $$
begin
    if p_filter = '' then
        begin
            return query
            select
                iso3166_1_numeric,
                official_name_en,
                iso3166_1_alpha_2,
                iso3166_1_alpha_3
            from common.countries a
            limit p_items
            offset p_page * p_items
            ;
        end
    else
        begin
            return query
            select
                iso3166_1_numeric,
                official_name_en,
                iso3166_1_alpha_2,
                iso3166_1_alpha_3
            from common.countries a
            where
                concat(
                    a.official_name_en,
                    a.iso3166_1_alpha_3
                ) like p_filter
            limit p_items
            offset p_page * p_items
            ;
        end
end
$$;