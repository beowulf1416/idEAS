create or replace procedure user_set_password (
    p_id iam.users.id%type,
    p_pw iam.users.pw%type
)
language plpgsql
as $$
begin
    update iam.users set
        pw = public.crypt(p_pw, public.gen_salt('md5'))
    where
        id = p_id
    ;
end
$$;