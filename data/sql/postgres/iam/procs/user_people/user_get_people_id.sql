create or replace function user_get_people_id(
    p_user_id iam.user_people.user_id%type
)
returns iam.user_people.people_id%type
language plpgsql
as $$
declare
    t_people_id iam.user_people.people_id%type;
begin
    select
        a.people_id
        into
        t_people_id
    from iam.user_people a
    where
        a.user_id = p_user_id
    ;

    return t_people_id;
end
$$;

comment on function user_get_people_id(iam.user_people.user_id%type) is 'retrieve associated people record';