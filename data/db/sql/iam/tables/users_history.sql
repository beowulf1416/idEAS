/**
 * users temporal table
 */
create table users_history (
    user_id uuid not null,
    active boolean not null,
    created timestamp without time zone not null default(now() at time zone 'utc'),
    email common.email_address not null,

    constraint fk_users_history
        foreign key (user_id)
        references iam.users (id)
        on delete restrict
        on update restrict
);