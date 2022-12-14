set schema 'inventory';


\ir procs/warehouses/warehouse_add.sql
\ir procs/warehouses/warehouse_update.sql
\ir procs/warehouses/warehouse_set_active.sql
\ir procs/warehouses/warehouse_fetch.sql

\ir procs/locations/location_add.sql
\ir procs/locations/location_update.sql
\ir procs/locations/location_set_active.sql

\ir procs/items/item_add.sql
\ir procs/items/item_update.sql
\ir procs/items/item_set_active.sql

\ir procs/item_locations/item_location_add.sql
\ir procs/item_locations/item_location_update.sql
\ir procs/item_locations/item_location_set_active.sql

\ir procs/item_balances/item_balance_add.sql
\ir procs/item_balances/item_balance_subtract.sql
\ir procs/item_balances/item_balance_fetch.sql