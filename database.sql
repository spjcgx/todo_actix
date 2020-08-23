drop table if exists todo_list;
CREATE TABLE todo_list (
    id serial primary key,
    title VARCHAR(150) not null
);

drop table if exists todo_item;
create table todo_item(
    id serial primary key,
    title VARCHAR(150) not null,
    checked boolean not null default false,
    list_id INTEGER not null,
    foreign key(list_id) references todo_list(id)
);

INSERT INTO todo_list(title) VALUES ('List 1'), ('List 2');
INSERT INTO todo_item(title, list_id) VALUES
    ('iTEM 1', 1),
    ('iTEM 2', 2),
    ('iTEM 3', 1);