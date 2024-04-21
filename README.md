```rust
use std::{sync::mpsc, thread};

use proconio::input;

fn main() {
    let (tx, rx) = mpsc::channel();

    let manager = thread::spawn(move || {
        tx.send(true).unwrap();
        loop {
            input! {
                signal: u32,
            }
            
            if signal == 0 {
                tx.send(false).unwrap();
                break;
            }
        }
    });

    thread::spawn(move || {
        loop {
            let flag = rx.try_recv().ok();
            match flag {
                Some(v) => {
                    if !v {
                        break;
                    } else {
                        continue;
                    }
                },
                None => continue,
            }
        }

        println!("out of loop at speaker");
    });

    manager.join().unwrap();
}
```

```rust
use std::io;

fn main() {
    let some_doing = |x: i32| -> Result<i32, io::Error>{
        if x == 0 {
            Ok(200)
        } else if x == 1{
            Ok(202)
        } else {
            Ok(10000)   
        }
    };

    match some_doing(235) {
        Err(e) => println!("{}", e),
        Ok(v) if v == 200 => println!("{}", v),
        Ok(v) if v == 202 => println!("{}", v),
        Ok(v) => println!("other {}", v), 
    }
}

// match gard
// match valiable {
//  Err() => ...,
//  Ok(v) if v == flag => ...,
//  Ok(v) if v == flag => ...,
//  Ok(v) => ...,
// }    
```