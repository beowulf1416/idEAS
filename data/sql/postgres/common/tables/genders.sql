create table genders (
    id smallint not null,
    name varchar(20) not null,

    constraint pk_genders
        primary key (id)
);

insert into genders (id, name) values 
(1, 'male'),
(2, 'female')
;