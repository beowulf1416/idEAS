create or replace procedure user_add(
    p_id iam.users.id%type,
    p_email iam.users.email%type
)
language plpgsql
as $$
declare
    t_salt text;
    t_pw text;
begin
    -- generate random password
    t_salt := public.gen_salt('md5');
    t_pw := public.crypt(random()::text, t_salt);

    insert into iam.users (
        id,
        email,
        pw
    ) values (
        p_id,
        p_email,
        t_pw
    );
end
$$;