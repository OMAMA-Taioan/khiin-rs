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
    c.weight desc

{limit}