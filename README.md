# Deppth

Small CLI tool to check the dependency size of a repository.

## Command Flags
`-c` or `--crate`: specifies a crate to be analysed.
`-r` or `--repository`: specifies a repository to be analysed. Excludes `-c`/`--crate`

## Bussiness Rules
 * Should have a flag for checking a crate.
 * Should have a flag for checking a GitHub repository.
 * Should have a flag for specifying if it should output the size or the dependency list(or both. It should return the size by default).
 * Should return the dependency tree size by default.
 * Making a crate query should exclude the possibility of making a repository query, and vice versa.
