/**
 * create app user script
 */
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