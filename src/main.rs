use std::thread::sleep;
use std::thread;
use std::time::Duration;
#[tokio::main]
async fn main() {
    println!("Hello before reading file");

    let h1 = tokio::spawn(async {
        let _file1_contents = read_from_file1();
    });

    let h2 = tokio::spawn(async {
        let _file2_contents = read_from_file2();
    });

    let _ = tokio::join!(h1, h2);
}


fn read_from_file1() -> String {
    sleep(Duration::new(4,0));
    println!("{:?}", "Processing File 1");
    String::from("Hello from file 1")
}



fn read_from_file2() -> String {
    sleep(Duration::new(2,0));
    println!("{:?}", "Processing File 2");
    String::from("Hello from file 2")
}