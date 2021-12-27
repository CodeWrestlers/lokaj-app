CREATE TABLE garbage_types (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    emoji CHARACTER(2) NOT NULL DEFAULT '🗑',
    language_code VARCHAR NOT NULL
);

INSERT INTO garbage_types VALUES
    (1, 'zmieszane', '⚫️', 'pl'),
    (2, 'papier', '🔵', 'pl'),
    (3, 'plastik i metal', '🟡', 'pl'),
    (4, 'bio', '🟤', 'pl'),
    (5, 'szkło', '🟢', 'pl')
;

CREATE TABLE garbage_collection (
    id SERIAL PRIMARY KEY,
    garbage_type_id SERIAL NOT NULL,
    collection_date DATE NOT NULL,

    FOREIGN KEY (garbage_type_id) REFERENCES garbage_types(id)
);

INSERT INTO garbage_collection (garbage_type_id, collection_date) VALUES
    (1, '2021-12-09'),
    (1, '2021-12-23'),
    (2, '2021-12-14'),
    (2, '2021-12-28'),
    (3, '2021-12-14'),
    (3, '2021-12-28'),
    (4, '2021-12-16'),
    (4, '2021-12-30'),
    (5, '2021-12-10'),
    (1, '2022-01-08'),
    (1, '2022-01-20'),
    (2, '2022-01-11'),
    (2, '2022-01-25'),
    (3, '2022-01-11'),
    (3, '2022-01-25'),
    (4, '2022-01-13'),
    (4, '2022-01-27'),
    (5, '2022-01-14')
;
