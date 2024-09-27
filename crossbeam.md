# crossbeam

## [epoch](https://docs.rs/crossbeam/latest/crossbeam/epoch/index.html)

keywords
- epoch-based garbage collection for concurrent collections
- atomic pointers
- epoch-protected pointer
- pinning
- global garbage queue
- guard: a guard that keeps current thread pinned

`epoch-based` memory reclamation
- an element gets removed from a concurrent collection, 
- it is inserted into a pile of garbage and marked with the current epoch.
- Every time a thread accesses a collection
  - it checks the current epoch, 
  - attempts to increment it, 
  - and destructs some garbage that became so old that no thread can be referencing it anymore.

memory reclamation is designed to be fully automatic 
and something users of concurrent collections don’t have to worry much about.

### guard


