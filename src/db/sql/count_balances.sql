SELECT COUNT(DISTINCT nickname)
FROM assets
WHERE balance_as_of <= ?;