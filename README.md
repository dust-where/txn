# txn

第一步:实现只加锁,而不去寻找环

```rust
use std::thread;
use std::time::Duration;

// fn read_begin_action(num: i32) {
//     println!("read_begin:{}", num);
//     // unsafe {
//     // }   
// }

// fn read_end_action(num: i32) {
//     println!("read_end:{}", num);
// }

// 存储两行数据,第一行是,第二行行数据是这个编号的数据再谁手上
static mut ARR: [[i32; 5]; 2] = [[0i32; 5]; 2];

fn write_begin_action(num: i32, thread_id: i32) -> bool{
    let mut check = true;
    unsafe {
        if ARR[1][num as usize] == 0 { // 能够放进去
            ARR[1][num as usize] = thread_id;
            println!("将 {} 资源给与 {} 成功", num, thread_id);
        } else { // 不能放进去 
            // 查找要等待的那个点
            let iptt = ARR[1][num as usize];
            // 将要等待的放入
            ARR[0][thread_id as usize] = iptt;
            check = false;
            println!("将 {} 资源给与 {} 失败, 因为 {} 线程正在占用", num, thread_id, iptt);
        }
    }
    check
}

fn write_end_action(num: i32, thread_id: i32) {
    unsafe {
        // 还回数据
        ARR[1][num as usize] = 0;
    } 
    println!("将 {} 资源还回 {} 成功", num, thread_id);
}

fn main() {
    let thread1 = thread::spawn(|| {
        // 要1,2,3
        let thread_id = 1;
        write_begin_action(1, thread_id);
        write_begin_action(2, thread_id);
        write_begin_action(3, thread_id);
        // 模拟操作
        thread::sleep(Duration::from_millis(2000));
        // 释放1,2,3
        write_end_action(1, thread_id);
        write_end_action(2, thread_id);
        write_end_action(3, thread_id);
    });

    let thread2 = thread::spawn(|| {
        // 要3,4,2
        let thread_id = 2;
        write_begin_action(3, thread_id);
        write_begin_action(4, thread_id);
        write_begin_action(2, thread_id);
        thread::sleep(Duration::from_millis(2000));
        write_end_action(3, thread_id);
        write_end_action(4, thread_id);
        write_end_action(2, thread_id);
    });

    let thread3 = thread::spawn(|| {
        // 要2,4,1
        let thread_id = 3;
        write_begin_action(2, thread_id);
        write_begin_action(4, thread_id);
        write_begin_action(1, thread_id);
        thread::sleep(Duration::from_millis(2000));
        write_end_action(2, thread_id);
        write_end_action(4, thread_id);
        write_end_action(1, thread_id);
    });

    let thread4 = thread::spawn(|| {
        // 要1.4
        let thread_id = 4;
        write_begin_action(1, thread_id);
        write_begin_action(4, thread_id);
        thread::sleep(Duration::from_millis(2000));
        write_end_action(1, thread_id);
        write_end_action(4, thread_id);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    thread4.join().unwrap();
}
```

暂时结果:

```
PS C:\Users\13378\Desktop\txn\txn> cargo run                                                                  Compiling txn v0.1.0 (C:\Users\13378\Desktop\txn\txn)                                                       Finished dev [unoptimized + debuginfo] target(s) in 0.76s                                                   Running `target\debug\txn.exe`                                                                        
将 4 资源给与 4 失败, 因为 3 线程正在占用
将 3 资源还回 2 成功
将 4 资源还回 2 成功
将 2 资源还回 2 成功
将 1 资源还回 1 成功
将 2 资源还回 1 成功
将 3 资源还回 1 成功
将 2 资源还回 3 成功
将 4 资源还回 3 成功
将 1 资源还回 3 成功
将 1 资源还回 4 成功
将 4 资源还回 4 成功
PS C:\Users\13378\Desktop\txn\txn> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\txn.exe`
将 1 资源给与 1 成功
将 2 资源给与 1 成功
将 3 资源给与 1 失败, 因为 2 线程正在占用
将 1 资源给与 4 失败, 因为 1 线程正在占用
将 4 资源给与 4 成功
将 2 资源给与 3 失败, 因为 1 线程正在占用
将 4 资源给与 3 失败, 因为 4 线程正在占用
将 1 资源给与 3 失败, 因为 1 线程正在占用
将 3 资源给与 2 成功
将 4 资源给与 2 失败, 因为 4 线程正在占用
将 2 资源给与 2 失败, 因为 1 线程正在占用
将 2 资源还回 3 成功
将 4 资源还回 3 成功
将 1 资源还回 3 成功
将 1 资源还回 1 成功
将 2 资源还回 1 成功
将 3 资源还回 1 成功
将 3 资源还回 2 成功
将 4 资源还回 2 成功
将 2 资源还回 2 成功
将 1 资源还回 4 成功
将 4 资源还回 4 成功
PS C:\Users\13378\Desktop\txn\txn>
```

把他改成死锁的程序

```rust
use std::thread;
use std::time::Duration;

// fn read_begin_action(num: i32) {
//     println!("read_begin:{}", num);
//     // unsafe {
//     // }   
// }

// fn read_end_action(num: i32) {
//     println!("read_end:{}", num);
// }

// 存储两行数据,第一行是,第二行行数据是这个编号的数据再谁手上
static mut ARR: [[i32; 5]; 2] = [[0i32; 5]; 2];

fn write_begin_action(num: i32, thread_id: i32) -> bool{
    let mut check = true;
    unsafe {
        if ARR[1][num as usize] == 0 { // 能够放进去
            ARR[1][num as usize] = thread_id;
            println!("将 {} 资源给与 {} 成功", num, thread_id);
        } else { // 不能放进去 
            // 查找要等待的那个点
            let iptt = ARR[1][num as usize];
            // 将要等待的放入
            ARR[0][thread_id as usize] = iptt;
            check = false;
            println!("将 {} 资源给与 {} 失败, 因为 {} 线程正在占用", num, thread_id, iptt);
        }
    }
    check
}

fn write_end_action(num: i32, thread_id: i32) {
    unsafe {
        // 还回数据
        ARR[1][num as usize] = 0;
    } 
    println!("将 {} 资源还回 {} 成功", num, thread_id);
}

fn main() {
    let thread1 = thread::spawn(|| {
        // 要1,2,3
        let thread_id = 1;
        let mut check = false;
        loop {
            check = write_begin_action(1, thread_id);
            if check == true {
                break
            }
        }
        loop {
            check = write_begin_action(2, thread_id);
            if check == true {
                break
            }
        }
        loop {
            check = write_begin_action(3, thread_id);
            if check == true {
                break
            }
        }
        // 模拟操作
        thread::sleep(Duration::from_millis(2000));
        // 释放1,2,3
        write_end_action(1, thread_id);
        write_end_action(2, thread_id);
        write_end_action(3, thread_id);
    });

    let thread2 = thread::spawn(|| {
        // 要3,4,2
        let thread_id = 2;
        let mut check = false;
        loop {
            check = write_begin_action(3, thread_id);
            if check == true {
                break
            }
        }
        loop {
            check = write_begin_action(4, thread_id);
            if check == true {
                break
            }
        }
        loop {
            check = write_begin_action(2, thread_id);
            if check == true {
                break
            }
        }
        thread::sleep(Duration::from_millis(2000));
        write_end_action(3, thread_id);
        write_end_action(4, thread_id);
        write_end_action(2, thread_id);
    });

    let thread3 = thread::spawn(|| {
        // 要2,4,1
        let thread_id = 3;
        let mut check = false;
        loop {
            check = write_begin_action(2, thread_id);
            if check == true {
                break
            }
        }
        loop {
            check = write_begin_action(4, thread_id);
            if check == true {
                break
            }
        }
        loop {
            check = write_begin_action(1, thread_id);
            if check == true {
                break
            }
        }
        thread::sleep(Duration::from_millis(2000));
        write_end_action(2, thread_id);
        write_end_action(4, thread_id);
        write_end_action(1, thread_id);
    });

    let thread4 = thread::spawn(|| {
        // 要1.4
        let thread_id = 4;
        let mut check = false;
        loop {
            check = write_begin_action(1, thread_id);
            if check == true {
                break
            }
        }
        loop {
            check = write_begin_action(4, thread_id);
            if check == true {
                break
            }
        }
        thread::sleep(Duration::from_millis(2000));
        write_end_action(1, thread_id);
        write_end_action(4, thread_id);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    thread4.join().unwrap();
}
```

