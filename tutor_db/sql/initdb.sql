create table course
(
    course_id   bigserial primary key,
    tutor_id    bigint       not null,
    course_name varchar(200) not null,
    created_at  timestamptz default CURRENT_TIMESTAMP not null
);
