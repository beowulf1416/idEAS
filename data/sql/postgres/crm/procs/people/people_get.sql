create or replace function people_get(
    people_id crm.people.id%type
) 
returns table (
    id crm.people.id%type,
    active crm.people.active%type,
    created crm.people.created%type,
    given_name crm.people.given_name%type,
    middle_name crm.people.middle_name%type,
    family_name crm.people.family_name%type,
    prefix crm.people.prefix%type,
    suffix crm.people.suffix%type
)
language plpgsql
as $$
begin
    return query
    select
        a.id,
        a.active,
        a.created,
        a.given_name,
        a.middle_name,
        a.family_name,
        a.prefix,
        a.suffix
    from crm.people a
    where
        a.id = people_id
    ;
end
$$;

comment on function people_get (crm.people.id%type) is 'retrieve people record';