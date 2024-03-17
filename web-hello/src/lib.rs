use std::thread;

pub struct ThreadPool {
    // thread::JoinHandle<T> 中的T,是闭包返回的类型
    // 这里暂时不需要任何返回值,所以使用()
    threads: Vec<thread::JoinHandle<()>>,
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
        let mut threads = Vec::with_capacity(size);
        for _ in 0..size {
            // create some threads and store them into the vector
        }
        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f: F) where F: FnOnce() + Send + 'static {}
}
