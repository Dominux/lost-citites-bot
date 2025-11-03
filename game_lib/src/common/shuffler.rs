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

pub struct NotShuffler;

impl Shuffler for NotShuffler {
    fn shuffle<T>(&self, _: &mut Vec<T>) {}
}
