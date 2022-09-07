create table ethnicity (
    id smallint not null,
    name varchar(30) not null,

    constraint pk_ethnicity
        primary key (id)
);

insert into ethnicity (id, name) values 
(1, 'african'),
(2, 'asian'),
(3, 'caucasian'),
(4, 'indian'),
(5, 'european')
;