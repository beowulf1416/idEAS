/**
 * add user account
 */
create or replace procedure user_add (
    p_user_id iam.users.id%type,
    p_email iam.users.email%type,
    p_pw iam.users.pw%type
)
language plpgsql
as $$
begin
    insert into iam.users (
        id,
        email,
        pw
    ) values (
        p_user_id,
        p_email,
        public.crypt(p_pw, public.gen_salt('md5'))
    );
end
$$;