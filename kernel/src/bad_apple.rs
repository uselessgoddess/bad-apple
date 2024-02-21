pub const APPLE: [&str; 6570] = {
    let mut place = [""; 6570];

    seq_macro::seq!(N in 1..=6570 {
        place[N - 1] = include_str!(concat!("../frames/", N, ".txt"));
    });

    place
};
