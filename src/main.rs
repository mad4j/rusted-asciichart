use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
}

const COLUMN_SEPARTOR: &str = " | ";

const LABELS: [&str; 33] = [
    "NUL", "SOH", "STX", "ETX", "EOT", "ENQ", "ACK", "BEL", 
    "BS ", "HT ", "LF ", "VT ", "FF ", "CR ", "SO ", "SI ",
    "DLE", "DC1", "DC2", "DC3", "DC4", "NAK", "SYN", "ETB", 
    "CAN", "EM ", "SUB", "ESC", "FS ", "GS ", "RS ", "US ",
    "DEL",    // 112..127
];


fn get_char_label(c: char) -> String {

    if c.is_ascii_control() {

        let i = c as usize;

        if i<32 {
            LABELS[i%32].to_string()
        } else if i == 127 {
            LABELS[32].to_string()
        } else {
            String::from("---")    
        }

    } else {
        String::from("---")
    }
}

fn get_char_name(c: char) -> String {
    if c.is_ascii() {
        // For printable ASCII characters, return the character itself as a string.
        if c.is_ascii_graphic() {
            c.to_string()
        }
        // For special ASCII characters like space, tab, newline, etc.
        // return a predefined name or symbol.
        else if c == ' ' {
            "SPC".to_string()
        } else if c == '\t' {
            "TAB".to_string()
        } else if c == '\n' {
            "LF".to_string()
        } else if c == '\r' {
            "CR".to_string()
        }
        // For other ASCII control characters, use existing LABELS
        // or a simple placeholder if not in LABELS.
        // This part can reuse or adapt logic from get_char_label if suitable,
        // or simply return a placeholder like "CTL".
        // For simplicity, returning "CTL" for other control characters.
        else if c.is_ascii_control() {
            // We can use the existing LABELS array here, similar to get_char_label
            let i = c as usize;
            if i < 32 {
                LABELS[i].trim().to_string() // Use trim to remove padding if any
            } else if i == 127 {
                LABELS[32].trim().to_string() // DEL character
            } else {
                "CTL".to_string() // Should not happen for ASCII control
            }
        }
        // Fallback for any other ASCII characters (e.g. DEL if not handled above)
        else {
            "ASC".to_string() // Generic ASCII
        }
    } else {
        // For non-ASCII characters, keep the existing logic or simplify.
        // The current code returns "." if s.starts_with("'\u{'"), which is a reasonable simplification.
        let s = format!("{c:#?}");
        if s.starts_with("'\\u{") {
            ".".to_string()
        } else {
            s // This case might include multi-char sequences for some Unicode chars.
        }
    }
}

fn print_char_info(c: char) {

    let i = c as usize;

    let n = get_char_name(c);

    let l = get_char_label(c);


    print!("{i:03} {i:02X} {i:03o} {n:^4} {l}");
}

fn print_char_table() {

    println!();

    for i in 0..64 {

        print!("{}", COLUMN_SEPARTOR);
        for j in 0..4 {
            let c = char::from_u32(i+j*64).unwrap();
            print_char_info(c);
            print!("{}", COLUMN_SEPARTOR);
        }
        println!();
    }
}

fn main() {

    let _args = Args::parse();

    print_char_table();
}
