create or replace function countries_fetch()
returns table (
    id common.countries.iso_3166_1_numeric%type,
    name common.countries.official_name_en%type,
    iso_3166_1_alpha_2 common.countries.iso_3166_1_alpha_2%type,
    iso_3166_1_alpha_3 common.countries.iso_3166_1_alpha_3%type
)
language plpgsql
as $$
begin
    return query
    select
        a.iso_3166_1_numeric,
        a.official_name_en,
        a.iso_3166_1_alpha_2,
        a.iso_3166_1_alpha_3
    from common.countries a
    ;
end
$$;