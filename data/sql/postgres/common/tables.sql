/**
 * identity and access management
 */
create schema common;
set schema 'common';


-- http://www.regular-expressions.info/email.html
create domain email_address
  text
  constraint chk_email
  check(
    length(value) < 254
    and
    value ~ '^[a-zA-Z0-9._%+-]{1,64}@(?:[a-zA-Z0-9-]{1,63}\.){1,125}[a-zA-Z]{2,63}$'
);


create domain slug_text
  text
  constraint chk_slug
  check(
    length(value) > 2
    and
    length(value) < 254
    and
    value ~ '[0-9a-z]'
);



-- tables
\ir tables/countries.sql
\ir tables/currencies.sql
\ir tables/dimensions.sql
\ir tables/uom.sql
\ir tables/genders.sql
\ir tables/ethnicity.sql
\ir tables/marital_states.sql


-- populate
-- https://www.postgresql.org/docs/12/app-psql.html
-- countries
\copy common.countries (official_name_en,iso_3166_1_alpha_2,iso_3166_1_alpha_3,iso_3166_1_numeric,iso_4217_currency_alphabetic_code,iso_4217_currency_country_name,iso_4217_currency_minor_unit,iso_4217_currency_name,iso_4217_currency_numeric_code) from '/docker-entrypoint-initdb.d/countries.csv' with delimiter ',' csv header quote '"'

-- currencies
\copy common.currencies (currency, unit_text, symbol) from '/docker-entrypoint-initdb.d/currencies.csv' with delimiter ',' csv header quote '"'; 