# threadpool

My, rather naive, implementation (see [main.rs](https://github.com/jinjagit/threadpool/blob/main/src/bin/main.rs)) of Ryan Levicks's Rust threadpool library (see [lib.rs](https://github.com/jinjagit/threadpool/blob/main/src/lib.rs)), from [his tutorial](https://www.youtube.com/watch?v=2mwwYbBRJSo).  

Learning to use Arc, Mutex, Atomic variables, and developing understanding of ownership, borrowing and closures.  

## compile to release version

Assuming you have Rust installed, compile to a _release build_ to reveal actual performance differences from sharing the calculation between different numbers of threads:
* clone repo.  
* `$ cd threadpool`  
* `$ cargo build --release`  
* `$ cd target/release`  
* `$ ./main`  

## example output

![threadpool.png](readme_img/threadpool.png)