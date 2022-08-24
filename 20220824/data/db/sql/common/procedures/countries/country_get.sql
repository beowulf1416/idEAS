create or replace function country_get (
    country_id common.countries.iso3166_1_numeric%type
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
    return query
    select
        iso3166_1_numeric,
        official_name_en,
        iso3166_1_alpha_2,
        iso3166_1_alpha_3
    from common.countries a
    where
        a.iso3166_1_numeric = country_id
    ;
end
$$;