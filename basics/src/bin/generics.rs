use rand::{distributions::Uniform, prelude::Distribution};

fn main() {
    let mut rng = rand::thread_rng();
    let rng_sampler = Uniform::from(1..=100);

    let v = (0..20)
        .map(|_| rng_sampler.sample(&mut rng))
        .collect::<Vec<_>>();

    assert_eq!(largest(&v), v.iter().max().unwrap());
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest_item_ref = &list[0];

    for item_ref in list {
        if item_ref > largest_item_ref {
            largest_item_ref = item_ref;
        }
    }

    largest_item_ref
}
