create table irrigate.plant
(
    id                      serial
        primary key,
    name                    varchar not null,
    plant_profile_id        integer not null
        references plant_profile,
    soil_moisture           real    not null,
    last_watered            real    not null,
    environment_humidity    real    not null,
    environment_temperature real    not null
);

create table irrigate.plant_profile
(
    id                          serial
        primary key,
    name                        varchar not null,
    min_soil_moisture           real    not null,
    min_period_between_watering real,
    watering_duration           integer not null,
    min_temperature             real,
    min_humidity                real,
    max_temperature             real,
    max_humidity                real
);