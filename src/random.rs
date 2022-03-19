use rand::Rng;

// Source: https://github.com/yewstack/yew/blob/master/examples/keyed_list/src/random.rs

/// half-open: [min, max)
pub fn range_exclusive(min: usize, max: usize) -> usize {
    let len: usize = rand::thread_rng().gen();
    len % (max - min) + min
}