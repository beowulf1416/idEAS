create table user_people(
    people_id uuid not null,
    user_id uuid not null,

    constraint pk_user_people
        primary key (people_id, user_id),

    constraint u_user_people_1
        unique (people_id),
    constraint u_user_people_2
        unique (user_id),

    constraint fk_user_people_1
        foreign key (people_id)
        references crm.people (id)
        on delete restrict,
    constraint fk_user_people_2
        foreign key (user_id)
        reference iam.users (id)
        on delete restrict
);

comment on table user_people is 'table mapping users to people records';