#[macro_export]
macro_rules! wbg {
    () => {
        {
            use std::io::Write;
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("debug.log")
                .unwrap();
            writeln!(
                f,
                "[{}:{}]",
                file!(),
                line!()
            ).unwrap();
        }
    };

    ($val:expr $(,)?) => {
        {
            use std::io::Write;
            let tmp = &$val;
            let mut f = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("debug.log")
                .unwrap();
            writeln!(
                f,
                "[{}:{}] {} = {:#?}",
                file!(),
                line!(),
                stringify!($val),
                tmp
            ).unwrap();
            tmp
        }
    };

    ($($val:expr),+ $(,)?) => {
        ($($crate::wbg!($val)),+,)
    };
}

#[macro_export]
macro_rules! clear_wbg {
    () => {
        {
            std::fs::File::create("debug.log").unwrap();
        }
    };
}
