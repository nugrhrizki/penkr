SELECT t.table_name AS collection,
    cast(array_agg(
        (
            c.name,
            c.ordinal_position,
            c.constraint,
            c.char_max_len,
            c.numeric_precision,
            c.numeric_scale,
            c.data_type,
            c.is_nullable,
            c.is_unique,
            c.is_auto_increment,
            c.default_value
        )
    ) as TEXT) AS "fields"
FROM information_schema.tables AS t
    LEFT JOIN (
        SELECT c.column_name AS name,
            c.ordinal_position,
            constraint_type AS constraint,
            character_maximum_length AS char_max_len,
            numeric_precision,
            numeric_scale,
            c.table_name,
            data_type,
            CASE
                WHEN is_nullable = 'YES' THEN true
                ELSE false
            END AS is_nullable,
            CASE
                WHEN constraint_type = 'UNIQUE' THEN true
                ELSE false
            END AS is_unique,
            CASE
                WHEN column_default LIKE 'nextval%' THEN true
                ELSE false
            END AS is_auto_increment,
            CASE
                WHEN column_default LIKE 'nextval%' THEN NULL
                ELSE column_default
            END AS default_value
        FROM information_schema.columns c
            LEFT JOIN information_schema.key_column_usage as kcu ON c.column_name = kcu.column_name
            AND c.table_name = kcu.table_name
            LEFT JOIN information_schema.table_constraints as tc ON kcu.constraint_name = tc.constraint_name
        ORDER BY c.ordinal_position ASC
    ) AS c ON c.table_name = t.table_name
WHERE t.table_schema = 'public'
    AND t.table_type = 'BASE TABLE'
    AND t.table_catalog = $1
GROUP BY t.table_name
ORDER BY t.table_name ASC
