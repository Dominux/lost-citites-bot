use rand::seq::SliceRandom;

pub trait Shuffler {
    fn shuffle<T>(&self, vec: &mut Vec<T>);
}

pub struct RandomShuffler;

impl Shuffler for RandomShuffler {
    fn shuffle<T>(&self, vec: &mut Vec<T>) {
        vec.shuffle(&mut rand::thread_rng())
    }
}
