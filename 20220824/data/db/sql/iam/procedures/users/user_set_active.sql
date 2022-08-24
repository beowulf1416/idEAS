/**
 * toggle user active status
 */
create or replace procedure user_set_active (
    p_id iam.users.id%type,
    p_active iam.users.active%type
)
language plpgsql
as $$
begin
    insert into iam.users_history (
        user_id,
        active,
        email
    )
    select
        p_id,
        active,
        email
    from iam.users
    where
        id = p_id
    ;

    update iam.users set
        active = p_active
    where
        id = p_id;
end
$$