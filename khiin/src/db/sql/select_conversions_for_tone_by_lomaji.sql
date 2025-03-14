select
    c.*

from
    conversion_lookups c

where
    c.key_sequence = :query
and c.input_type = :input_type
and c.{khin_mode} = 1

order by
    c.is_hanji asc,
    c.weight desc

{limit}