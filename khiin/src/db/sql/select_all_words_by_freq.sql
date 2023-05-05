select
    input_id,
    key_sequence,
    input_type,
    n_syls,
    p
from
    key_sequences
where
    input_type = ? or
    input_type = 0 -- Toneless
order by
    p desc
