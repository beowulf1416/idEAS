create or replace procedure user_add(
    p_id iam.users.id%type,
    p_email iam.users.email%type,
    p_given_name iam.users.given_name%type,
    p_family_name iam.users.family_name%type,
    p_prefix iam.users.honorific_prefix%type,
    p_suffix iam.users.honorific_suffix%type
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
        pw,
        given_name,
        family_name,
        honorific_prefix,
        honorific_suffix
    ) values (
        p_id,
        p_email,
        t_pw,
        p_given_name,
        p_family_name,
        p_prefix,
        p_suffix
    );
end
$$;