-- Platform API
DROP TABLE IF EXISTS users CASCADE;
DROP TABLE IF EXISTS user_sessions CASCADE;
DROP TABLE IF EXISTS channel_tags CASCADE;
DROP TABLE IF EXISTS recordings CASCADE;
DROP TABLE IF EXISTS categories CASCADE;
DROP TABLE IF EXISTS image_uploads CASCADE;
DROP TABLE IF EXISTS chat_messages CASCADE;
DROP TABLE IF EXISTS global_state CASCADE;
DROP TABLE IF EXISTS roles CASCADE;
DROP TABLE IF EXISTS channel_user CASCADE;
DROP TYPE IF EXISTS image_type;

-- Image Processor
DROP TABLE IF EXISTS users image_jobs;
