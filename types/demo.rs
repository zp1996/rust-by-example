use::std::collections::HashMap;

fn main() {
    let arr = [11, 12, 11, 13, 13, 11, 10, 1, 2, 3, 5, 1];
    println!("{:?}", get_number_info(&arr));

    println!("{}", );
}

fn get_number_info(arr: &[i32]) -> (f64, f64, i32) {
    let len = arr.len();
    let half = len / 2;
    let mut cache = HashMap::new();
    let mut count = 0;
    let mut max = 0;
    let mut max_key = arr[0];
    let mut vec = Vec::new();

    for item in arr.iter() {
        let k = *item;

        let mut n = cache.entry(k).or_insert(0);
        *n += 1;

        if *n >= max {
            max = *n;
            max_key = k;
        }

        count += k;
        vec.push(k);
    }

    let mid = if len % 2 == 0 {
        ((*&vec[half] + *&vec[half - 1]) as f64) / 2.0
    } else {
        *&vec[half] as f64
    };

    let average = (count as f64) / (arr.len() as f64);
    return (average, mid, max_key);
}

fn to_pig_latin(s: &str) -> String {
    let res = &s[1..];
    println!("{}", res);
    return res.to_string();
}
