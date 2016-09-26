macro_rules! running {
    () => (RUNNING.load(Ordering::Relaxed));
    ($val: expr) => (RUNNING.store($val, Ordering::Relaxed));
}

macro_rules! quit {
    ($msg: expr) => {
        println!("{}", $msg);
        process::exit(0)
    };
    ($msg: expr, $code: expr) => {
        println!("{}", $msg);
        process::exit($code)
    };
}
