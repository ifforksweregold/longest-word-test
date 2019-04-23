use std::time;

pub fn main() {
    //let filename = env::args().nth(1).unwrap();
    //let file: std::string::String = fs::read_to_string(filename).expect("");
    //let words = file.par_split_whitespace();
    
    let words = "There are four words".to_string();

    let start = time::Instant::now();

    let split = words.split_whitespace();

    let elapsed = start.elapsed();
    let seconds = ((elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0)) * 1000.0;
    println!("`.split_whitespace` found {} words in {} milliseconds", split.len(), seconds);

    //words = "There are four words".to_string;

    //let start = time::Instant::now();


    //let elapsed = start.elapsed();
    //let seconds = ((elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0)) * 1000.0;
    //println!("`.split_whitespace` found {} words in {} milliseconds", split.len(), seconds);
}
