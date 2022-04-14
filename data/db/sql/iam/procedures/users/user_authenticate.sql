/**
 * authenticate user
 */
create or replace function user_authenticate (
    p_email iam.users.email%type,
    p_pw iam.users.pw%type
)
returns boolean
language plpgsql
as $$
declare
    t_authentic boolean;
begin
    select
        a.pw = public.crypt(p_pw, a.pw)
        into
        t_authentic
    from iam.users a
    where
        a.email = p_email
    ;

    return t_authentic;
end
$$;