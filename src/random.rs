extern crate rand;
use rand::Rng;

pub fn select_random<T: Copy>(xs: &[T]) -> T {
    let random_index = rand::thread_rng().gen_range(0, xs.len());
    xs[random_index]
}
