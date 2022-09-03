create or replace function countries_fetch(
    p_filter varchar(100),
    p_items int,
    p_page int
)
returns table (
    id common.countries.iso3166_1_numeric%type,
    name common.countries.official_name_en%type,
    iso3166_1_alpha_2 common.countries.iso3166_1_alpha_2%type,
    iso3166_1_alpha_3 common.countries.iso3166_1_alpha_3%type
)
language plpgsql
as $$
begin
    
end
$$;