static INPUT: &str = "yzbqklnj";

fn main() {
    let goal = "000000";
    let mut c = 1;

    loop {
        let test_string = format!("{}{}", INPUT, c);

        let digest = md5::compute(test_string);
        let digest = format!("{:?}", digest);
        if &digest[0..6] == goal {
            break;
        }
        c += 1;
    }

    dbg!(c);
}
