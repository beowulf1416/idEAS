create or replace procedure user_people_add(
    p_user_id crm.user_people.user_id%type,
    p_people_id crm.user_people.people_id%type
)
language plpgsql
as $$
begin
    insert into crm.people (
        user_id,
        people_id
    ) values (
        p_user_id,
        p_people_id
    );
end
$$;