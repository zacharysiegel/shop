-- Refinery runs SQL scripts within transaction blocks. This prevents the use of CREATE DATABASE here so the following
-- must be executed manually. See init.sql.

drop role if exists authelia;
create role authelia
    nosuperuser
    nocreatedb
    login
    noreplication
    nobypassrls
    password 'IOv/Anp95KZPERCAYNecgDPdlmsxSRkj0vBjgssskF8'
-- todo: rotate this password and set it in a setup script
;

alter database authelia
    owner to authelia
;
