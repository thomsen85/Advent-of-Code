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
