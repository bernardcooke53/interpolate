# Interpolate

For the purposes of this algorithm & program all indices are 0-based,
so a 3x3 matrix has indices 0..2 in each axis.

## Algorithm walkthrough
Using the data from `tests/test_data/test1.csv`, an example of the algorithm at work is as follows:

The input data looks like this:
```
[1   , None, 2   , 7   , 9   , 12  , None],
[None, None, 4   , None, 6   , 9   , 0],
[7   , 4   , 9   , 1   , 5   , 0   , 6],
[10  , 1   , None, 7   , 4   , 3   , 8],
[None, 9   , 4   , 2   , None, None, None],
[None, None, 6   , 4   , None, 2   , 1],
[None, 2   , 2   , 3   , 4   , 5   , 6],
```

This means we will keep track of the None values not yet interpolated in a HashSet which is initially
```
{
    (0, 1), (0, 6),
    (1, 0), (1, 1), (1, 3),
    (3, 2),
    (4, 0), (4, 4), (4, 5), (4, 6),
    (5, 0), (5, 1), (5, 4),
    (6, 0)
}
```

Start with the top-leftmost None value, at index (0, 1). The first non-None value
to the left is 1 (index (0, 0)), below is 4 (index (2, 1)), and to the right is 2 (index (0, 2)).

We average these values to get (4+2+1)/3 = 2.333333, and update our matrix with this
value in position (0, 1).

We continue left-to-right, as the HashSet remembers its insertion order and we inserted
the indices originally by iterating left-to-right, top-to-bottom. This means position (0, 6) gets
updated to (12+0)/2 = 6.

After filling the first row of this matrix, it now looks like

```
[1        , 2.333333 , 2       , 7       ,    9       , 12      , 6],
[None     , None     , 4       , None    ,    6       , 9       , 0],
[7        , 4        , 9       , 1       ,    5       , 0       , 6],
[10       , 1        , None    , 7       ,    4       , 3       , 8],
[None     , 9        , 4       , 2       ,    None    , None    , None],
[None     , None     , 6       , 4       ,    None    , 2       , 1],
[None     , 2        , 2       , 3       ,    4       , 5       , 6],
```

When we continue to the second row, we fill position (1, 0) with (1+7+4)/3 = 4,
as normal. However, when we fill position (1, 1), we *use* the value in position (1, 0)
as the first non-None value to the left - meaning we fill (1, 1) with the caluclation
(4+4+4+4)/4 = 4. This is how the algorithm approaches adjacent None values.

Finishing out the algorithm in this way produces the following matrix:

```
[1        , 2.333333 , 2        , 7        , 9        , 12       , 6        ],
[4        , 3.583333 , 4        , 4.5      , 6        , 9        , 0        ],
[7        , 4        , 9        , 1        , 5        , 0        , 6        ],
[10       , 1        , 5.25     , 7        , 4        , 3        , 8        ],
[9.5      , 9        , 4        , 2        , 3.333333 , 2.777778 , 3.925926 ],
[7.75     , 6.1875   , 6        , 4        , 3.333333 , 2        , 1        ],
[4.875    , 2        , 2        , 3        , 4        , 5        , 6        ],
```
