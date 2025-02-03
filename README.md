
# On Linux:

`cargo run`

works!

`cargo run  --features=socketpair` 

works!

# On Windows

I _suspect_ that

`cargo run`

will work, but

`cargo run --features=socketpair`

will deadlock.
