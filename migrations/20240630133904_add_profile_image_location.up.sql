-- Add up migration script here
ALTER TABLE user_profile
ADD COLUMN image_location VARCHAR(255);