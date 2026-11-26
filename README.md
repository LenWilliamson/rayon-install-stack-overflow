# rayon-install-stack-overflow

In my [Chapaty](https://github.com/LenWilliamson/chapaty) crate, I use rayon to perform a grid search over different agent configurations. To reduce the memory overhead I maintain a heap of the top `k` performing agents. I frequently ran into a stack ovreflow error, even though I didn't use deep recursion:

```sh
thread '<unknown>' (896130) has overflowed its stack
fatal runtime error: stack overflow, aborting
zsh: abort      cargo run
```

It turned out, that rayon caused the stack overflow. Inside my fold, I called functions from [Polars](https://crates.io/crates/polars), which call [`ThreadPool::install`](https://docs.rs/rayon/latest/rayon/struct.ThreadPool.html#method.install). This kept my rayon workers busy and because my code for maintaining the heap was in some cases slower than the agents finishing their work, rayon's work stealing algorithm kept pulling work which caused the stack to overflow. I fixed this problem in commit [a3dea37](https://github.com/LenWilliamson/chapaty/commit/a3dea37c492d198da7d20166732042c44b2edd41). And all versions `>= 1.4.x` of [Chapaty](https://github.com/LenWilliamson/chapaty) don't have this bug.

This is a minimal reproduction of a stack overflow in Rayon without recursion: `ThreadPool::install` in another pool makes a waiting worker steal jobs onto its own stack. To break the cycle I use a MPMC channel and build a custom `std::thread` worker pool where I send the work through a bounded channel. Because the `std::thread` pool is unrelated to `rayon` it is safe to call `ThreadPool::install`.
