#[macro_export]
macro_rules! log {
  ($fmt:expr) => {{
    print!("[{}] ", chrono::Local::now().format("%Y/%m/%d %H:%M:%S"));
    print!($fmt);
  }};
  ($fmt:expr, $($arg:tt)*) => {{
    print!("[{}] ", chrono::Local::now().format("%Y/%m/%d %H:%M:%S"));
    print!($fmt, $($arg)*);
  }};
}

#[macro_export]
macro_rules! elog {
  ($fmt:expr) => {{
    eprint!("[{}] ERR: ", chrono::Local::now().format("%Y/%m/%d %H:%M:%S"));
    eprint!($fmt);
  }};
  ($fmt:expr, $($arg:tt)*) => {{
    eprint!("[{}] ERR: ", chrono::Local::now().format("%Y/%m/%d %H:%M:%S"));
    eprint!($fmt, $($arg)*);
  }};
}

#[macro_export]
macro_rules! logln {
  () => {
    println!();
  };
  ($fmt:expr) => {{
    print!("[{}] ", chrono::Local::now().format("%Y/%m/%d %H:%M:%S"));
    println!($fmt);
  }};
  ($fmt:expr, $($arg:tt)*) => {{
    print!("[{}] ", chrono::Local::now().format("%Y/%m/%d %H:%M:%S"));
    println!($fmt, $($arg)*);
  }};
}

#[macro_export]
macro_rules! elogln {
  () => {
    eprintln!();
  };
  ($fmt:expr) => {{
    eprint!("[{}] ERR: ", chrono::Local::now().format("%Y/%m/%d %H:%M:%S"));
    eprintln!($fmt);
  }};
  ($fmt:expr, $($arg:tt)*) => {{
    eprint!("[{}] ERR: ", chrono::Local::now().format("%Y/%m/%d %H:%M:%S"));
    eprintln!($fmt, $($arg)*);
  }};
}
