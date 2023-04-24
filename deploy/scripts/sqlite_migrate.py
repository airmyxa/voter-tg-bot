import sqlite3
import typing
import os
import argparse


class Settings:
    MIGRATIONS_PATH = './migrations/sqlite/'


class Migrations:
    def __init__(self, migration_data: typing.Dict):
        self.versions = sorted(migration_data.keys(), key=lambda x: int(x[1:]))
        self.migrations_data = migration_data


def read_migration_file(filename):
    f = open(Settings.MIGRATIONS_PATH + filename)
    return f.read()


def apply_migrations(cursor, migrations: Migrations, baseline: int):
    for migration_version in migrations.versions:
        version = int(migration_version[1:])
        if version <= baseline:
            print(f'skipping migration {migration_version} because baseline {baseline} is greater or equal')
            continue
        print(f'applying migration: {migrations.migrations_data[migration_version]["execute_command"]}')
        cursor.execute(migrations.migrations_data[migration_version]["execute_command"])
        set_baseline(cursor, version)


def get_migrations() -> Migrations:
    migration_files = os.listdir(Settings.MIGRATIONS_PATH)
    result = {}
    for migration in migration_files:
        (version, migration_name) = migration.split('__')
        result[version] = {}
        cur = result[version]
        cur['name'] = migration_name.split('.')[0]
        cur['execute_command'] = read_migration_file(migration)
    return Migrations(result)


def get_baseline(cursor) -> int:
    return cursor.execute('PRAGMA user_version').fetchone()[0]


def set_baseline(cursor, new_baseline: int):
    cursor.execute(f'PRAGMA user_version = {new_baseline}')


def parse_args():
    parser = argparse.ArgumentParser()
    parser.add_argument('--connection_file')
    return parser.parse_args()


def main():
    args = parse_args()
    migrations = get_migrations()
    connection = sqlite3.connect(args.connection_file)
    cursor = connection.cursor()
    baseline = get_baseline(cursor)
    apply_migrations(cursor, migrations, baseline)


if __name__ == '__main__':
    main()
