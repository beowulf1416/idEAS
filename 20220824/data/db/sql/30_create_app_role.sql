/**
 * create app user script
 */
-- create procedure create_app_role()
-- language plpgsql
-- as $$
-- begin
--     if exists(
--         select
--             *
--         from pg_user
--         where
--             usename = 'saas_app_usr'
--     ) then
--         begin
            create role saas_app_usr
            with
            nocreatedb
            noinherit
            login
            noreplication
            nobypassrls
            connection limit 2
            password 'saas_app_usr_pw'
            ;
--         end;
-- end;
-- $$;

-- call create_app_role();
-- drop procedure create_app_role;