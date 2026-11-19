-- Add migration script here
-- CMS SCHEMA 
GRANT USAGE ON SCHEMA cms TO app_user;
-- Set default privileges for future tables created by migrator
ALTER DEFAULT PRIVILEGES FOR ROLE migrator IN SCHEMA cms 
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO app_user;
ALTER DEFAULT PRIVILEGES FOR ROLE migrator IN SCHEMA cms 
GRANT USAGE, SELECT ON SEQUENCES TO app_user;
-- Grant permissions on existing tables
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA cms TO app_user;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA cms TO app_user;

-- USERS SCHEMA 
GRANT USAGE ON SCHEMA users TO app_user;
-- Set default privileges for future tables created by migrator
ALTER DEFAULT PRIVILEGES FOR ROLE migrator IN SCHEMA users 
GRANT SELECT, INSERT, UPDATE, DELETE ON TABLES TO app_user;
ALTER DEFAULT PRIVILEGES FOR ROLE migrator IN SCHEMA users 
GRANT USAGE, SELECT ON SEQUENCES TO app_user;
-- Grant permissions on existing tables
GRANT SELECT, INSERT, UPDATE, DELETE ON ALL TABLES IN SCHEMA users TO app_user;
GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA users TO app_user;


