use hls_parser::parse_default_hls;

fn main() {
    if let Err(e) = parse_default_hls() {
        println!("Error: {:?}", e);
    }
}
