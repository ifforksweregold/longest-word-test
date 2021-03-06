# Longest Word Test

## The Rules

1. Solution may be in any language and use any implementation or algorithm
1. Solution must find the longest word in a corpus. Words are only considered to be space and newline delimited text.
1. Solution is timed from first loading/reading of the corpus to the return of the longest word and/or length of the word.
  - Parsing arguments is not counted
  - Runtime initialization is not counted
  - Time should be measured in MS

Note: Solutions are currently divided into two categories: single-threaded and concurrent.

## How to participate

Write your solution, add your name to the top, and send a pull request. It would be super awesome if you also updated the Dockerfile and the `run_tests.sh` to include your submission, but that's not necessary.

## Leaderboard

### Single-Threaded

| Name             | Language   | Time (ms) | Notes                                |
|------------------|------------|-----------|--------------------------------------|
| @dnesbitt03qub   | C++        | 418       | Striding implementation              |
| @dnesbitt03qub   | C++        | 675       | Simple optimized                     |
| @pard68          | Rust       | 1066      | Simple                               |
| @pard68          | Rust       | 1069      | HashTable                            |
| Jeffrey Corcoran | C#         | 2275      |                                      |
| @bbriggs         | Go         | 2849      |                                      |
| @zombeej         | JavaScript | 3941      |                                      |
| @pard68          | Rust       | 4033      | Pythonic                             |
| @pard68          | Rust       | 5760      | Striding                             |
| @drewpearce      | Cython     | 6581      | Loop w/ longest                      |
| @specs           | Python     | 12245     | Comprehensions                       |
| @samwi           | JRuby      | 13565     |                                      |
| @specs           | Python     | 14444     | Loop and sort                        |
| @specs           | Python     | 16257     | Loop w/ longest                      |
| @samwi           | Ruby       | 24383     |                                      |
| @patrickcarver   | Elixir     | 47510     |                                      |
| @pard68          | Bash       | ~5543     | Native speed - Not running in Docker |


### Concurrent
| Name             | Language | Time (ms) | Notes             |
|------------------|----------|-----------|-------------------|
| @pard68          | Rust     | 197       | HashTable         |
| Jeffrey Corcoran | C#       | 716       |                   |
| @pard68          | Rust     | 2016      | Pythonic          |
| @samwi           | JRuby    | 4679      | 'Multithread 16.' |
| @samwi           | Ruby     | 55339     | 'Multithread 8.'  |


### Methodology
