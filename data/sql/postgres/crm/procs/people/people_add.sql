create or replace procedure people_add(
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
    insert into crm.people (
        id,
        active,
        given_name,
        middle_name,
        family_name,
        prefix,
        suffix
    ) values (
        p_people_id,
        true,
        p_given_name,
        p_middle_name,
        p_family_name,
        p_prefix,
        p_suffix
    );
end
$$;