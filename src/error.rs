use quick_error::quick_error;

quick_error! {
    #[derive(Debug)]
    pub enum HexxdError {
        Io(err: std::io::Error) {
            from()
            display("I/O Error: {}", err)
        }
        FromUtf8(err: std::string::FromUtf8Error) {
            from()
            display("Utf-8 Error: {}", err)
        }
        ParseInt(err: std::num::ParseIntError) {
            from()
            display("Int Parse Error: {}", err)
        }
    }
}
