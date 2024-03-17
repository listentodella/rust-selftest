use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
}

impl ThreadPool {
    /// 创建线程池
    ///
    /// `size`:线程池中的数量
    ///
    /// # Panics
    ///
    /// `new`函数在size为0时会panic
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        //with_capacity与new做了同样的工作，不过他为vector预先分配空间
        //而new随着后面push元素需要重新改变大小,效率上不如with_capacity
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i));
        }
        ThreadPool { workers }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        Worker {
            id,
            thread: thread::spawn(|| {}),
        }
    }
}
