select
    c.*

from
    conversion_lookups c

where
    c.key_sequence = :query
and (
    c.input_type = :input_type
)

order by
    c.is_hanji desc,
    c.weight desc

{limit}