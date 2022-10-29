create or replace procedure user_people_update(
    p_user_id iam.user_people.user_id%type,
    p_people_id iam.user_people.people_id%type
)
language plpgsql
as $$
begin
    insert into iam.user_people (
        user_id,
        people_id
    ) values (
        p_user_id,
        p_people_id
    );
end
$$;

comment on procedure user_people_update(iam.user_people.user_id%type, iam.user_people.people_id%type) is 'associate user record to people record';