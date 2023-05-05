with cte as (
    select
        c.*,
        row_number() over (
            partition by c.output
            order by
                c.weight desc,
                length(c.key_sequence) desc
        ) as rn
    from
        conversion_lookups c
    where
        c.key_sequence in ({vars})
        and (
            c.input_type = {input_type}
            or c.input_type = 0
        )
)
select *
from cte
where rn = 1
order by
length(cte.key_sequence) desc,
(cte.weight / cte.n_syls) desc
--,
-- cte.weight desc
