create or replace procedure people_update(
    p_people_id crm.people.id%type,
    p_given_name crm.people.given_name%type,
    p_middle_name crm.people.middle_name%type,
    p_family_name crm.people.family_name%type,
    p_prefix crm.people.prefix%type,
    p_suffix crm.people.suffix%type
)
language plpgsql
as $$
begin
    update crm.people set
        given_name = p_given_name,
        middle_name = p_middle_name,
        family_name = p_family_name,
        prefix = p_prefix,
        suffix = p_suffix
    where
        people_id = p_people_id
    ;
end
$$;