use clap::Parser;
use std::io::Read;

/// This is a simple program
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// It just works!
    #[clap(long, short, action)]
    cbytes: bool,
    #[clap(long, short, action)]
    lines: bool,
    #[clap(long, short, action)]
    mchars: bool,
    #[clap(long, short, action)]
    words: bool,

    filename: Option<String>,
}

struct FileMetrics {
    words: i16,
    bytes: i16,
    characters: i16,
    lines: i16,
}

impl FileMetrics {
    pub fn new<T: Read>(file: T) -> FileMetrics {
        let mut metrics = FileMetrics {
            words: 0,
            bytes: 0,
            characters: 0,
            lines: 0,
        };

        metrics.analyze(file);

        return metrics;
    }

    pub fn stringFromArgs(&self, cbytes: bool, lines: bool, mchars: bool, words: bool) -> String {
        let no_flags = !cbytes && !lines && !mchars && !words;
        let mut output = String::new();

        if cbytes || no_flags {
            output = format!("{} {}", output, self.bytes);
        }

        if lines || no_flags {
            output = format!("{} {}", output, self.lines);
        }

        if words || no_flags {
            output = format!("{} {}", output, self.words);
        }

        if mchars {
            output = format!("{} {}", output, self.characters)
        }

        return output;
    }

    fn analyze<T: Read>(&mut self, file: T) {
        let mut in_word = false;

        for byte_result in file.bytes() {
            in_word = match byte_result {
                Ok(byte) => self.process_byte(byte, in_word),
                _ => in_word,
            };
        }
    }

    fn process_byte(&mut self, byte: u8, in_word: bool) -> bool {
        self.bytes += 1;

        let mut is_current_in_word = in_word;
        if byte >> 6 == u8::from_le(2) {
            return is_current_in_word;
        }

        self.characters += 1;

        if !byte.is_ascii_whitespace() {
            is_current_in_word = true;
        } else if in_word {
            self.words += 1;
            is_current_in_word = false;
        }

        if byte == b'\n' {
            self.lines += 1;
            is_current_in_word = false;
        }

        return is_current_in_word;
    }
}

fn main() {
    let args = Args::parse();
    let filename: Option<String> = args.filename;

    let file: Box<dyn Read> = match filename.as_ref() {
        Some(value) => Box::new(std::fs::File::open(value).expect("Cannot open file")),
        None => Box::new(std::io::stdin()),
    };

    let wc = FileMetrics::new(file);

    let mut output = wc.stringFromArgs(args.cbytes, args.lines, args.mchars, args.words);

    match filename {
        Some(filename) => output = format!("{} {}", output, filename),
        None => {}
    }

    print!("{}\n", output);
}
