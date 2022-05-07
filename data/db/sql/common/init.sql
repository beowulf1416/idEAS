copy common.currencies (
    currency_id,
    name,
    symbol
)
from '../../csv/currencies.csv'
delimiter ','
csv
header
;