/**
 * accounting schema
 */
create schema accounting;
set schema 'accounting';

\ir tables/account_types.sql
\ir tables/accounts.sql