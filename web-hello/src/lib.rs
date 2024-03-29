use std::sync::{Arc, mpsc, Mutex};
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

// 来存放用于向信道中发送的闭包
type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        //with_capacity与new做了同样的工作，不过他为vector预先分配空间
        //而new随着后面push元素需要重新改变大小,效率上不如with_capacity
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender: Some(sender) }
    }

    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 丢弃sender会关闭信道，表明不会有更多的消息被发送
        // 这会造成worker中的无限循环中的所有recv()都会返回错误
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down the worker {}", worker.id);
            // 如果worker的线程是None,则此时这个worker已经清理了其线程,不需再做任何操作
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}


struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc::<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }

            //let job = receiver.lock().unwrap().recv().unwrap();
            //job();

            // while let, if let, match 是代码块,在代码块结束前都不会丢弃临时值
            // 这就意味着这种方式,job()在调用期间将一直持锁,其他worker无法接受任务处理
            // while let Ok(job) = receiver.lock().unwrap().recv() {
            //     println!("Worker {id} got a job; executing.");
            //     job();
            // }
        });
        Worker {
            id,
            thread: Some(thread),
        }
    }
}
