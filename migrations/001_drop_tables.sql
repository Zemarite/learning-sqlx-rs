-- DO $$ 
-- DECLARE 
--     r RECORD;
-- BEGIN 
--     FOR r IN (SELECT tablename FROM pg_tables WHERE schemaname = 'public' AND tablename != '_sqlx_migrations') LOOP
--         EXECUTE 'DROP TABLE IF EXISTS ' || quote_ident(r.tablename) || ' CASCADE';
--     END LOOP; 
-- END $$

drop extension if exists "pgcrypto";
-- drop table if exists _sqlx_migrations cascade;
drop table if exists member_groups cascade;
drop table if exists members cascade;
drop table if exists groups cascade;
drop table if exists divisions cascade;
drop table if exists organizations cascade;
