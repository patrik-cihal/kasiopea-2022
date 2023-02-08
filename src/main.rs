use std::io::Write;

mod a;
mod b;
mod c;
mod d;
mod e;
mod g;

type Input = token_read::TokenReader<std::io::StdinLock<'static>>;

pub static mut OUTPUT: Option<std::io::BufWriter<std::io::Stdout>> = None;
pub static mut INPUT: Option<Input> = None;

fn main() {
    let (t, ): (usize, ) = scanln();
    for _ in 0..t {
        g::solve();
    }
    flush();
}

#[macro_export]
macro_rules! out {
    ($($arg:tt)*) => {{
        let output = unsafe {
            OUTPUT.get_or_insert_with(|| std::io::BufWriter::with_capacity(1024*1024, std::io::stdout()))
        };
        write!(output, $($arg)*).unwrap();
    }};
}

#[macro_export]
macro_rules! outln {
    ($($arg:tt)*) => {{
        let output = unsafe {
            OUTPUT.get_or_insert_with(|| std::io::BufWriter::with_capacity(1024*1024, std::io::stdout()))
        };
        writeln!(output, $($arg)*).unwrap();
    }};
}

fn flush() {
    let output = unsafe {
        OUTPUT.get_or_insert_with(|| std::io::BufWriter::with_capacity(1024*1024, std::io::stdout()))
    };
    output.flush().unwrap();
}

fn scanln<T: token_read::FromTokens>() -> T where <T as token_read::FromTokens>::Error: std::fmt::Debug {
    let input = unsafe {
        INPUT.get_or_insert_with(|| Input::new(std::io::stdin().lock()))
    };
    input.line().unwrap()
}

fn scanlns<T: token_read::FromTokens + std::fmt::Debug>(n: usize) -> std::iter::Map<token_read::Take<'static, T, std::io::StdinLock<'static>>, fn(Result<T, token_read::ReadTokensError<<T as token_read::FromTokens>::Error>>) -> T> where <T as token_read::FromTokens>::Error: std::fmt::Debug {
    let input = unsafe {
        INPUT.get_or_insert_with(|| Input::new(std::io::stdin().lock()))
    };
    input.take::<T>(n).map(|x| x.unwrap())
}

fn scanln_raw() -> String {
    let input = unsafe {
        INPUT.get_or_insert_with(|| Input::new(std::io::stdin().lock()))
    };
    input.line_raw().unwrap()
}
