-- Step 1: Update the default value for privacy_level
ALTER TABLE users_user
ALTER COLUMN privacy_level
SET DEFAULT 2;

-- Step 2: Update existing records to ensure all values are within the allowed range
UPDATE users_user
SET
    privacy_level = 2
WHERE
    privacy_level NOT IN (0, 1, 2, 3);

-- Step 3: Add a constraint to limit values of privacy_level
ALTER TABLE users_user
ADD CONSTRAINT chk_privacy_level CHECK (privacy_level IN (0, 1, 2, 3));