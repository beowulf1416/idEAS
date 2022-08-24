create or replace function user_fetch(
    p_filter varchar(100),
    p_items int default 10,
    p_page int default 0
)
returns table (
    id iam.users.id%type,
    active iam.users.active%type,
    email iam.users.email%type,
    given_name iam.users.given_name%type,
    family_name iam.users.family_name%type,
    prefix iam.users.honorific_prefix%type,
    suffix iam.users.honorific_suffix%type
)
language plpgsql
as $$
begin
    if p_filter = '' then
        return query
        select
            a.id,
            a.active,
            a.email,
            a.given_name,
            a.family_name,
            a.honorific_prefix,
            a.honorific_suffix
        from iam.users a
        order by
            a.email
        limit p_items
        offset p_items * p_page
        ;
    else
        return query
        select
            a.id,
            a.active,
            a.email,
            a.given_name,
            a.family_name,
            a.honorific_prefix,
            a.honorific_suffix
        from iam.users a
        where
            concat_ws(
                '-',
                a.email,
                a.given_name,
                a.family_name,
                a.honorific_prefix,
                a.honorific_suffix
            ) like p_filter
        order by
            a.email
        limit p_items
        offset p_items * p_page
        ;
    end if;
end
$$;