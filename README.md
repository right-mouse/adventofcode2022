# Advent of Code 2022

My attempt at <https://adventofcode.com/2022>. To run, use:

```sh
cargo run -r [-- [-d <day>] [-i <input>]]
```

Optional arguments:

- `day`, if specified, returns the solution only for the particular day.
  Otherwise, the solution for all days is returned.
- `input` can be specified to a different directory from the default value of
  "inputs". The expected directory structure is `<input>/<day>/input`. For
  example, the input for day 3 at the default path would be `inputs/3/input`.
  If `day` is specified, `input` can also be the path to the input file itself.
