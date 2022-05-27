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


\ir tables/countries.sql
\ir tables/currencies.sql
\ir tables/dimensions.sql
\ir tables/uom_quantity.sql
\ir tables/uom_length.sql
\ir tables/uom_area.sql
\ir tables/uom_volume.sql
\ir tables/uom_mass.sql