\copy common.countries from '../../csv/countries.csv' with csv header

\copy common.currencies from '../../csv/currencies.csv' with csv header

-- copy common.countries (
--     official_name_en,
--     iso3166_1_alpha_2,
--     iso3166_1_alpha_3,
--     iso3166_1_numeric,
--     iso4217_currency_alphabetic_code,
--     iso4217_currency_country_name,
--     iso4217_currency_minor_unit,
--     iso4217_currency_name,
--     iso4217_currency_numeric_code
-- )
-- from '../../csv/countries.csv'
-- delimiter ','
-- csv
-- header
-- ;

-- copy common.currencies (
--     currency_id,
--     name,
--     symbol
-- )
-- from '../../csv/currencies.csv'
-- delimiter ','
-- csv
-- header
-- ;