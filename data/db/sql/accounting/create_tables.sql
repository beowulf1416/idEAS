/**
 * accounting schema
 */
create schema accounting;
set schema 'accounting';

\ir tables/tenants.sql

\ir tables/account_types.sql
\ir tables/accounts.sql
\ir tables/account_balances.sql