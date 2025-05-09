-- Refinery runs SQL scripts within transaction blocks. This prevents the use of CREATE DATABASE here so the following
-- must be executed manually. See init.sql.

drop role authelia;
create role authelia
    nosuperuser
    nocreatedb
    login
    noreplication
    nobypassrls
    password 'IOv/Anp95KZPERCAYNecgDPdlmsxSRkj0vBjgssskF8'
;

alter database authelia
    owner to authelia
;
