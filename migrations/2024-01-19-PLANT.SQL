create table plant_profile (
    id serial primary key,
    name varchar(64) not null,
    max_moisture float4 not null,
    min_moisture float4 not null
);

create table plant (
    id serial primary key,
    name varchar(64) not null ,
    profile_id int4 not null references plant_profile(id),
    last_watered timestamp,
    uuid varchar(36) not null,
    soil_moisture float4 not null default 0
);