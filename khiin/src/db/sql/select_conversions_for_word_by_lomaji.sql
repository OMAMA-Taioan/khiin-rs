select
    c.*

from
    conversion_lookups c

where
    (c.key_sequence = :query and c.input_type = :input_type and c.n_syls = 1) 
    or (c.key_sequence = :detoned_query and c.input_type = 0 and c.n_syls > 1)

order by
    c.is_hanji asc,
    c.weight desc

{limit}