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

static mut KILL: i32 = 0;

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

// 如果进入,那就确定现在可能是锁定状态,返回那个环上的任意一个节点
fn dfs(arr: [[i32; 5]; 2]) -> i32 {
    let mut res = 0;
    for i in 0..5 {
        if arr[0][i] == 0 {
            continue;
        } else {
            let mut now: [bool; 5] = [true; 5]; // 存储这个节点,向下寻找,如果找到一个false,把这个线程关闭
            now[i] = false;
            let mut num1: i32 = i as i32;   // 记录现在所在节点
            let mut check = false;
            loop { // 向下搜寻,如果不是环就跳出循环
                if check == true {      // 不是环
                    break;
                } else {
                    num1 = arr[0][num1 as usize];
                    if now[num1 as usize] == false {    // 是环
                        check = true;
                        res = num1;
                    } else {            // 向下推导
                        now[num1 as usize] = false;
                    }
                }
            }
            if res != 0 {
                break;
            }
        }
    }
    res
}

fn main() {
    let thread1 = thread::spawn(|| {
        // 要1,2,3
        let thread_id = 1;
        let mut check = false;
        println!("{}", check);
        loop {
            check = write_begin_action(1, thread_id);
            if check == true {
                break
            }
            unsafe {
                if KILL == 1 {
                    break;
                }
            }
        }
        loop {
            check = write_begin_action(2, thread_id);
            if check == true {
                break
            }
            unsafe {
                if KILL == 1 {
                    break;
                }
            }
        }
        loop {
            check = write_begin_action(3, thread_id);
            if check == true {
                break
            }
            unsafe {
                if KILL == 1 {
                    break;
                }
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
        println!("{}", check);
        loop {
            check = write_begin_action(3, thread_id);
            if check == true {
                break
            }
            unsafe {
                if KILL == 2 {
                    break;
                }
            }
        }
        loop {
            check = write_begin_action(4, thread_id);
            if check == true {
                break
            }
            unsafe {
                if KILL == 2 {
                    break;
                }
            }
        }
        loop {
            check = write_begin_action(2, thread_id);
            if check == true {
                break
            }
            unsafe {
                if KILL == 2 {
                    break;
                }
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
        println!("{}", check);
        loop {
            check = write_begin_action(2, thread_id);
            if check == true {
                break
            }
            unsafe {
                if KILL == 3 {
                    break;
                }
            }
        }
        loop {
            check = write_begin_action(4, thread_id);
            if check == true {
                break
            }
            unsafe {
                if KILL == 3 {
                    break;
                }
            }
        }
        loop {
            check = write_begin_action(1, thread_id);
            if check == true {
                break
            }
            unsafe {
                if KILL == 3 {
                    break;
                }
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
        println!("{}", check);
        loop {
            check = write_begin_action(1, thread_id);
            if check == true {
                break
            }
            unsafe {
                if KILL == 4 {
                    break;
                }
            }
        }
        loop {
            check = write_begin_action(4, thread_id);
            if check == true {
                break
            }
            unsafe {
                if KILL == 4 {
                    break;
                }
            }
        }
        thread::sleep(Duration::from_millis(2000));
        write_end_action(1, thread_id);
        write_end_action(4, thread_id);
    });

    // 检测是否有死锁出现
    let thread_detect_deadlock = thread::spawn(|| {
        loop { // 一直检测死锁
            thread::sleep(Duration::from_millis(1000)); // 等开始再检测

            // 查看死锁能否结束
            let mut check = true; 
            for i in 1..5 {
                unsafe {
                    if ARR[0][i as usize] != 0 {
                        check = false;
                        break;
                    }
                }
            }
            if check == true {
                break;
            }

            // 如果不能结束,就查看是否有环,如果有环,就把这个线程给毙掉(因为不会让这个线程等会再开) 
            unsafe {
                let my_arr: [[i32; 5]; 2] = ARR.clone(); // 拷贝一个副本
                let thread_name = dfs(my_arr);
                match thread_name {
                    0 => {
                        // 暂时没锁
                        continue
                    }
                    1 => {
                        println!("杀死线程1");
                        KILL = 1;
                    }
                    2 => {
                        println!("杀死线程2");
                        KILL = 2;
                    }
                    3 => {
                        println!("杀死线程3");
                        KILL = 3;
                    }
                    4 => {
                        println!("杀死线程4");
                        KILL = 4;
                    }
                    _ => {
                        continue
                    }
                }
            }
        }
        println!("死锁检测完毕");
    } );

    thread1.join().unwrap();
    thread2.join().unwrap();
    thread3.join().unwrap();
    thread4.join().unwrap();
    thread_detect_deadlock.join().unwrap();
    
}