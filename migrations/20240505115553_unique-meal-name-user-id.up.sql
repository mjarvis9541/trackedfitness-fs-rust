-- Step 1: Identify duplicates
SELECT
    user_id,
    name,
    COUNT(*)
FROM
    meal
GROUP BY
    user_id,
    name
HAVING
    COUNT(*) > 1;

-- Step 2: Delete duplicates
DELETE FROM meal
WHERE
    id NOT IN (
        SELECT
            id
        FROM
            (
                SELECT
                    id,
                    ROW_NUMBER() OVER (
                        PARTITION BY
                            user_id,
                            name
                        ORDER BY
                            created_at
                    ) AS rn
                FROM
                    meal
            ) AS sub
        WHERE
            rn = 1
    );

-- Step 3: Add unique constraint for user_id and name
ALTER TABLE meal
ADD CONSTRAINT unique_user_id_name UNIQUE (user_id, name);