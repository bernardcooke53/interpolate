# Interpolate

## Summary
A command-line utility to fill in missing values in a 2D matrix supplied in a CSV file.
The interpolation algorithm replaces a missing value in a position with the average of
the closest non-missing values to the left, below, above and to the right. It begins
with the top-leftmost missing value and works left-to-right, top-to-bottom.

The algorithm uses interpolated values in subsequent calculations for other missing values.

## Usage
The source code was compiled & tested with the following tools:
- cargo 1.58.0 (f01b232bc 2022-01-19)
- rustc 1.58.1 (db9d1b20b 2022-01-20)

The dependencies are specified in Cargo.toml, will be installed when
compiling the source code. To compile, from the project root run
```sh
cargo build --release
```

The binary located at `${PROJECT_ROOT}/target/release/interpolate` can then be
used as follows:
```
./target/release/interpolate <path_to_your_file>.csv
```

A new CSV file with the repaired matrix will be created *in the same directory* as your input file.


## Brief

### User Story

As a data analyst, I can call a command-line tool that accepts a 
two-dimensional matrix as input and produces an output matrix with the same 
dimensions in which missing values have been interpolated as the average of all 
non-diagonal neighbouring values. 

### Task

Create a production quality software tool to implement the above user story. 

We have spoken to the user representative and they agreed that the initial task 
should be to focus on core functionality, so we have refined the functional 
requirements to a limited set which should provide them with a minimum viable 
product: 
  - Input is in the form of a comma-separated file (CSV) in which rows are 
    separated by newline characters and the value of each column within a row 
    is separated by a comma.
  - Missing values in the input are encoded as `nan`.
  - Output is in the same form as the input, and have the same number of rows 
    and columns, but should contain no missing values.
  - Missing values should be interpolated as the average of all neighbouring 
    non-diagonal values.
  - Values that are non-missing in the input should be preserved in the output 
    without change.

Our user representative have also provided a tiny example of input and output 
data files and we have included these in the 'example_data' directory.

The tool should be developed with acceptance and unit tests that would aid 
further collaborative development. Please use whatever language(s) you feel are 
most appropriate for this task, considering both what you are comfortable using 
and what would be the easiest for the team to continue to develop and maintain. 
At a minimum, include a README.md file that specifies how to run the code 
(including required versions of compiler/interpreter and how to install any 
required dependencies).

The purpose of this task is for you to showcase your approach to software 
development. Please do not spend more than three hours on this task.

### Extra

If you complete the task in less than three hours, consider adding the following 
additional requirements in the remaining time:
  - Exit gracefully and with user-friendly error messages when malformed input is
    provided.
  - Handle adjacent missing values (the choice of how to interpolate in this 
    case is left to you).


## Algorithm walkthrough

For the purposes of this algorithm & program all indices are 0-based,
so a 3x3 matrix has indices 0..2 in each axis.

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
