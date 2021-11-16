use is_in::IsIn;

fn main() {
    let slice1: [u8; 3] = [0, 3, 2];
    println!("{}", 2_u8.is_in(&slice1));
}