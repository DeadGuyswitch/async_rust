fn main() {
   println!("Before reading the file");
   let file_contents = read_from_file();
   println!("{:?}", file_contents);
   println!("Hello after reading file");
}

fn read_from_file() -> String {
    String::from("Hello World")
}
