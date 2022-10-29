create table user_people (
    user_id uuid not null,
    people_id uuid not null,

    constraint pk_user_people
        primary key (user_id, people_id),

    constraint u_user_people_1
        unique (user_id),
    constraint u_user_people_2
        unique (people_id),

    constraint fk_user_people_1
        foreign key (user_id)
        references iam.users (id)
        on delete restrict,
    constraint fk_user_people_2
        foreign key (people_id)
        references crm.people (id)
        on delete restrict
);

comment on table user_people is 'table mapping users to crm.people';