\cd docker-entrypoint-initdb.d

\copy common.countries from 'common/csv/countries.csv' with csv header

\copy common.currencies from 'common/csv/currencies.csv' with csv header