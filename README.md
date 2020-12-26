# benchtime

So you wrote a Rust app, and it is kind of slow. 
Either in the sense that your software is actually unusable, or you are just procrastinating by prematurely optimizing your code.

In any case, this script is here to help you. It shows you which parts of your code took the longest to execute.

## Utility

This little script works via small benchmark points/timestamps you sprinkle throughout your code, similar to debug statements.
Having done that, you get a tabular output of how long each part of your software took to run.

Example:

```
+------------------------------------+------------------------------------+----------------+
| Start                              | End                                | Duration in ms |
+------------------------------------+------------------------------------+----------------+
| Program Starts!                    | Calculation of Prime Number starts | 1              |
+------------------------------------+------------------------------------+----------------+
| Calculation of Prime Number starts | Calculation of Prime Number ends   | 3887           |
+------------------------------------+------------------------------------+----------------+
| Calculation of Prime Number ends   | Program Ends!                      | 0              |
+------------------------------------+------------------------------------+----------------+

```

## Usage

This repository contains a small `main.rs` exemplifying the usage. Basically, it works like this:

1. Initialization:

```let mut bm = benchmark::BenchmarkStorage::init();```

2. Add measuring points - do that as often as you want:

```bm.add("Program Starts!");```

3. Render the table output:

```bm.render()```

## FAQ

### Why not Bencher/Criterion/...?

"Classic" benchmarking tools are very cool, and Rust's are certainly not lacking. However, they work similar to tests.
Meaning you need to replicate the flow of your data in your benchmarking functions. This gets you a lot more clean and
meaningful results and a lot of advantages, but it is a lot of effort as well. Especially if you are just hunting for
slow math functions or loops that run too long. *benchtime* just works with your existing code.

### Can I adapt the code to work better with my project?

Probably yes. The script is very lightweight and based on a very simple data structure. E.g. if you want to create several
`BenchmarkStorage` or plot the output or something like that, no problem.

### Does it have feature X?

Probably not. As said, the software is very minimal as-is. However, feel free to extend and adapt. I accept pull requests.

### Why not a crate?

I currently lack the time and motivation to make and maintain one. Feel free to do so yourself if you want.