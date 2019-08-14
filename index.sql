drop schema if exists web cascade;
create schema web;
set schema 'web';

create function test(a jsonb) returns jsonb as $$
declare

begin
    return jsonb_build_object('.name', current_timestamp, 'c', 3333, 'ts', current_timestamp)  || a;
end;
$$ language plpgsql;