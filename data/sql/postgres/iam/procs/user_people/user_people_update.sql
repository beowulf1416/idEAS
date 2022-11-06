create or replace procedure user_people_update(
    p_user_id iam.user_people.user_id%type,
    p_given_name crm.people.given_name%type,
    p_middle_name crm.people.middle_name%type,
    p_family_name crm.people.family_name%type,
    p_prefix crm.people.prefix%type,
    p_suffix crm.people.suffix%type
)
language plpgsql
as $$
declare
    t_people_id iam.user_people.people_id%type;
begin
    t_people_id := iam.user_get_people_id(p_user_id);

    if t_people_id is null then
        call crm.people_add(
            p_user_id,
            p_given_name,
            p_middle_name,
            p_family_name,
            p_prefix,
            p_suffix
        );

        insert into iam.user_people (
            user_id,
            people_id
        ) values (
            p_user_id,
            p_user_id
        );
    else
        call crm.people_update(
            p_user_id,
            p_given_name,
            p_middle_name,
            p_family_name,
            p_prefix,
            p_suffix
        );
    end if;
end
$$;

comment on procedure user_people_update(
    iam.user_people.user_id%type,
    p_given_name crm.people.given_name%type,
    p_middle_name crm.people.middle_name%type,
    p_family_name crm.people.family_name%type,
    p_prefix crm.people.prefix%type,
    p_suffix crm.people.suffix%type
) is 'associate user record to people record';