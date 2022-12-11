# Advent of Code 2022

My attempt at <https://adventofcode.com/2022>. To run, use:

```sh
cargo +nightly run -r [-- [-d <days>] [-i <input>]]
```

Optional arguments:

- `days`, if specified, returns the solution only for the particular day(s).
  Otherwise, the solution for all days is returned. Comma separated lists and
  ranges with `-` are allowed. Some valid examples are:
  - `4`
  - `1,3,5`
  - `2-12`
  - `1-3,5-7,9-11,13`
- `input` can be specified to a different directory from the default value of
  "inputs". The expected directory structure is `<input>/<day>/input`. For
  example, the input for day 3 at the default path would be `inputs/3/input`.
  If a single `day` is specified, `input` can also be the path to the input
  file itself.
