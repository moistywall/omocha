#![allow(dead_code)]

mod rayt_mod;
mod code1;
mod code2;
mod code3;

fn main() {
    let mut no = 0;
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        no = args[1].parse::<i32>().unwrap();
    }
    run(no);
}

fn run(no: i32) {
    match no {
        0 => code1::run(),
        1 => code2::run(),
        2 => code3::run(),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // cargo test all -- --nocapture --ignored
    // If to check the rendered image, comment out line 5 in window.rs
    #[test] #[ignore]
    fn all() {
        for i in 100..=311 {
            println!("code{} running...", i);
            run(i);

            // let output_path = std::path::Path::new("render.png");
            // if output_path.exists() {
            //     let rename_path = std::path::Path::new("render").join(format!("render{}.png", i));
            //     std::fs::rename("render.png", rename_path).unwrap();
            // }
        }
    }

    #[test] #[ignore]
    fn main() {
        run(0);
    }
}