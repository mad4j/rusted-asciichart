use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Daniele Olmisani")]
#[command(version)]
#[command(long_about = "Dump an old-fashioned ASCII chart on OS shell.")]
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
    let s = format!("{c:#?}");

    if s.starts_with("'\\u{") { ".".to_string() } else { s }
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
