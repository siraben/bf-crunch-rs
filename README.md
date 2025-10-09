# Rust Port of BF-Crunch

This repository is a port of the legendary
[BF-Crunch](https://github.com/primo-ppcg/BF-Crunch) project that was
used in this [StackExchange code
golf](https://codegolf.stackexchange.com/questions/55422/hello-world/163590#163590)
to find the shortest BF at the time to output hello world. Here I'm
mostly using [Codex](https://chatgpt.com/codex) to automate the port,
and adding extra features and performance improvements over the
original. The first commit aims to be just the port of the program to
Rust without any behavioral changes.

When possible, new features are added behind flags so as to not break
compatibility with the original.


## Description


The solver specializes in BF programs whose initialization prefix follows this
template:
```
{...s2}<{s1}<{s0}[{k0}[<{j0}>{j1}>{c0}>{c1}>{c2...}<<<]{h}>{k1}]
```

The notation breaks the prefix into reusable segments:

- **`s`-segment** (`{...s2}<{s1}<{s0}`) seeds the tape with a stack of additive
  adjustments, each delimited by `[` or `<`, that prepares the loop structure.
- **`k` segments** (`{k0}` / `{k1}`) tweak the loop entry and exit cells by
  applying signed runs of `+` or `-` instructions before and after the main
  loop.
- **`j`-segment** (`<{j0}>{j1}`) performs the loop zipping motion that copies
  values across the tape.
- **`c`-segment** (`>{c0}>{c1}>{c2...}`) describes the core recurrence that
  distributes scaled values to neighboring tape cells.
- **`h` adjustment** (`{h}`) rebalances the loop counter after distributing
  the `c`-segment contributions.

The shortest useful program of this type has length 14:
```
+[[<+>->++<]>]
```
which computes the powers of 2 as _f<sub>n</sub> = 2*f<sub>n-1</sub>_, _f<sub>0</sub> = 1_

## Usage

```
Crunches BF programs to produce a given text.

Usage: bfcrunch [--options] text [limit]

Arguments:
  <text>   The text to produce.
  [limit]  The maximum BF program length to search for. If empty, the length of the shortest program found so far will be used (-r). Default = (empty)

Options:
  -i, --max-init <#>       The maximum length of the initialization segment. If empty, the program will run indefinitely. Default = (empty)
  -I, --min-init <#>       The minimum length of the initialization segment. Default = 14
  -t, --max-tape <#>       The maximum tape size to consider. Programs that utilize more tape than this will be ignored. Default = 1250
  -T, --min-tape <#>       The minimum tape size to consider. Programs that utilize less tape than this will be ignored. Default = 1
  -n, --max-node-cost <#>  The maximum cost for any node. Default = 20
  -l, --max-loops <#>      The maximum number of iterations of the main loop. Default = 30000
  -s, --max-slen <#>       The maximum length of the s-segment. Default = (empty)
  -S, --min-slen <#>       The minimum length of the s-segment. Default = 1
  -c, --max-clen <#>       The maximum length of the c-segment. Default = (empty)
  -C, --min-clen <#>       The minimum length of the c-segment. Default = 1
  -r, --rolling-limit      If set, the limit will be adjusted whenever a shorter program is found.
  -u, --unique-cells       If set, each used cell used for output will be unique.
      --full-program       Print the full BF program for each solution.
  -?, --help               Display this help text.
```

## Output

Output is given in three lines:
 1. Total length of the program found, and the initialization segment.
 2. Path taken, starting with the current tape pointer. Each node
    corresponds to one character of output, represented as (pointer,
    cost).
 3. Utilized tape segment.
 
For example, the final result for `bfcrunch "hello world" 70 -r -i23` is:
```
64: ++++[[<+>->+++++>+<<]>]
49, (45, 5), (44, 3), (45, 6), (45, 1), (45, 4), (42, 4), (43, 5), (45, 3), (45, 4), (46, 2), (44, 4)
32, 116, 100, 104, 108, 132, 0, 0, 132, 0
```
This corresponds to the full program:
```
++++[[<+>->+++++>+<<]>]<<<<.<+.>++++..+++.<<<.>+++.>>.+++.>.<<-.
```

## Printing full program
If you use the flag `--full-program` it will also show the complete
programs, instead of just the offsets.

```
$ bfcrunch "hello world" 70 -r -i23 --full-program
init-len: 14; limit: 70
init-len: 15; limit: 70
init-len: 16; limit: 70
init-len: 17; limit: 70
init-len: 18; limit: 70
init-len: 19; limit: 70
71: -[[<++>->->---<<]>+]<<<<<----.---.<--..<<<---.<<<++.>+++.>>.+++.>>>.>-.
79: -[--[<+>->+>>+<<<]>]<<<<<++.---.<<<<<<<<<<<<<<<<-..<<<.<<.>++.>.+++.>>>.>>>+++.
77: -[[<++>->+>+++<<]->]<<<<+.---.>-----..<<<<<<--.<<<<+.>>>>>--.<.+++.>>>>>>.<-.
78: -[[<+>->->>--<<<]>+]<<<<<++++.---.+++++++..+++.>-.>---.<<.+++.------.--------.
79: -[[<+>->++>+++<<]>+]<<<<<<<+++++++.---.>>--..+++.<-----.<<+.>>>.+++.------.<<-.
78: -[[<+>->+>>---<<<]>]<<++++++++.---.<+..+++.<.>++++++++.--------.+++.------.>-.
192:
+[[<+>->+++<]---->+]<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<.---.+++++++..+++.<<----.>++.>.+++.>>>-.--------.
```
