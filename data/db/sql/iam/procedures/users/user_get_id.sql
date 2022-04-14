/**
 * get user id
 */
create or replace function user_get_id (
    p_email iam.users.email%type
)
returns iam.users.id%type
language plpgsql
as $$
declare
    t_user_id iam.users.id%type;
begin
    select
        a.id
    from iam.users a
    where
        a.email = p_email
    ;

    return t_user_id;
end
$$