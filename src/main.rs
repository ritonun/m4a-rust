use m4a;

fn main() {
    let filepath = PathBuf::from("dev/sample1.m4a");
    m4a::read(filepath);
}
