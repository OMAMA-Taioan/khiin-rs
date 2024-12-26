select
    c.*

from
    conversion_lookups c

--    left join bigram_freq b
--        on b.lgram = :lgram
--        and c.output = b.rgram

    left join unigrams u
        on c.output = u.gram

where
    c.key_sequence = :query
and (
    c.input_type = :input_type
    or c.input_type = 0
)

order by
    c.is_hanji desc,
    u.n desc,
    c.weight desc

{limit}