pub fn string_to_t_grid<T: std::str::FromStr>(input: &str, seperator: &str) -> Vec<Vec<T>> {
    input
        .lines()
        .map(|line| {
            line.split(seperator)
                .map(|a| {
                    a.parse::<T>().unwrap_or_else(|_err| {
                        panic!("String to grid couldn't parse {:?} to given type", a)
                    })
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn string_to_t_vec<T: std::str::FromStr>(input: &str) -> Vec<T> {
    input
        .lines()
        .map(|line| {
            line.parse::<T>().unwrap_or_else(|_err| {
                panic!("String to vetor couldn't parse {:?} to given type", line)
            })
        })
        .collect()
}

pub fn string_to_char_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

pub fn string_to_single_int_grid(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .unwrap_or_else(|| panic!("{c:?} is not a base10 digit"))
                        as i32
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

// If a string contains a lot of padding this function will just retrive all the numbers and parse
// them into type t
pub fn string_to_extracted_nums_t_vec<T: std::str::FromStr>(input: &str) -> Vec<T> {
    let mut nums = Vec::new();
    let mut new_num = true;
    for c in input.chars() {
        if c.is_ascii_digit() || c == '-' {
            if new_num {
                nums.push(String::new());
            }
            nums.last_mut().unwrap().push(c);
            new_num = false;
        } else {
            new_num = true;
        }
    }

    nums.into_iter()
        .map(|num| {
            num.parse::<T>()
                .unwrap_or_else(|_err| panic!("Couldn't parse {:?} to given type", num))
        })
        .collect()
}
