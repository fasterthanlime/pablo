use crossbeam_utils::thread;
use rayon::prelude::*;
use std::collections::HashMap;

// reduced for readable output
const THREADS: usize = 2;

fn sanitize_content(thread: usize, content: &str) -> String {
    format!("thread-{} {}", thread, content.replace("bad", ""))
}

fn generate_map(sanitized: &str) -> HashMap<&str, usize> {
    let mut map = HashMap::new();

    for word in sanitized.split_ascii_whitespace() {
        map.entry(word).and_modify(|x| *x += 1).or_insert(1);
    }

    map
}

fn main() {
    crossbeam_threads_example();
    rayon_example();
}

fn crossbeam_threads_example() {
    let content: String = "big bad input string".into();

    let strings: Vec<_> = thread::scope(|s| {
        let mut results = Vec::new();
        for i in 0..THREADS {
            let content = &content;
            results.push(s.spawn(move |_| sanitize_content(i, content)));
        }
        results
            .into_iter()
            .map(|h| h.join())
            .collect::<Result<_, _>>()
    })
    .unwrap()
    .unwrap();

    // now, `main()` owns all the sanitized strings
    // so we can lend them to some worker threads again:

    let maps: Vec<_> = thread::scope(|s| {
        let mut results = Vec::new();
        for input in strings.iter() {
            results.push(s.spawn(move |_| generate_map(input)))
        }
        results
            .into_iter()
            .map(|h| h.join())
            .collect::<Result<_, _>>()
    })
    .unwrap()
    .unwrap();

    dbg!(("crossbeam threads", &strings, &maps));
}

fn rayon_example() {
    let content: String = "big bad input string".into();

    let strings: Vec<_> = (0..THREADS)
        .into_par_iter()
        .map(|i| sanitize_content(i, &content))
        .collect();

    let maps: Vec<_> = strings.par_iter().map(|s| generate_map(s)).collect();

    dbg!(("rayon", &strings, &maps));
}
