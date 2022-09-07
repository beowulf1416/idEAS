create or replace procedure people_set_active(
    p_people_id crm.people.id%type,
    p_active crm.people.active%type
)
language plpgsql
as $$
begin
    insert into crm.people (
        people_id,
        active,
        given_name,
        middle_name,
        family_name,
        prefix,
        suffix
    )
    select
        p_people_id,
        active,
        given_name,
        middle_name,
        family_name,
        prefix,
        suffix
    from crm.people
    where
        id = p_people_id
    ;

    update crm.people set
        active = p_active
    where
        id = p_people_id
    ;
end
$$;