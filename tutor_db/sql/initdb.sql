create table course
(
    course_id   bigserial primary key,
    tutor_id    bigint       not null,
    course_name varchar(200) not null,
    course_description varchar(2000),
    course_format varchar(30),
    course_structure varchar(200),
    course_duration varchar(30),
    course_price INT,
    course_language varchar(30),
    course_level varchar(30),
    created_at  timestamptz default CURRENT_TIMESTAMP not null
);
