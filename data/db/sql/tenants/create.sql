/**
 * tenant management
 */
create schema tenants;
set schema 'tenants';

\ir tables/tenants.sql

\ir procedures/tenant_add.sql
\ir procedures/tenant_set_active.sql
\ir procedures/tenant_default_id.sql