# rayon-install-stack-overflow
Minimal reproduction of a stack overflow in Rayon without recursion: ThreadPool::install into another pool makes a waiting worker steal unrelated jobs onto its own stack.
