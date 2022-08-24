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
\ir tables/genders.sql
\ir tables/ethnicity.sql
\ir tables/marital_states.sql


-- procs