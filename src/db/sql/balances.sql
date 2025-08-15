SELECT *
FROM assets
WHERE balance_as_of <= ?
WINDOW
    my_window AS (PARTITION BY nickname ORDER BY balance_as_of DESC)
QUALIFY
    row_number() OVER my_window == 1
ORDER BY nickname ASC
LIMIT ?
OFFSET ?;