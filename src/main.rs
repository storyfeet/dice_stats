use gobble::*;
use std::collections::BTreeMap;
use std::io::Write;

fn p_int() -> impl Parser<i64> {
    maybe(s_tag("-"))
        .then(read_fs(is_num, 1))
        .try_map(|(neg, ns)| {
            let mut n = ns
                .parse::<i64>()
                .map_err(|_| ECode::SMess("Too long int"))?;
            if neg.is_some() {
                n = -n
            }
            Ok(n)
        })
}

fn parse_dice() -> impl Parser<Vec<Vec<i64>>> {
    repeat_until(
        maybe(p_int())
            .then_ig(s_tag("["))
            .then(sep_until(p_int(), s_tag(","), s_tag("]"))),
        s_(eoi),
    )
    .map(|v| {
        let mut res = Vec::new();
        for (nop, ar) in v {
            match nop {
                Some(n) => {
                    for _ in 0..n {
                        res.push(ar.clone());
                    }
                }
                _ => res.push(ar),
            }
        }
        res
    })
}

fn main() {
    let stdin = std::io::stdin();
    loop {
        print!(">>");
        std::io::stdout().flush().ok();
        let mut s = String::new();
        match stdin.read_line(&mut s) {
            Ok(0) => {
                println!("All done");
                return;
            }
            _ => {}
        }
        match parse_dice().parse_s(&s.trim()) {
            Ok(r) => {
                println!("Parsed dice = {:?}", r);
                let p = calc_probs(&r);
                print_probabilities(&p);
                //println!("probabilities = {:?}", p);
            }
            Err(e) => println!("Err parsing dice: {:?}", e),
        }
    }
    //println!("Hello, world!");
}

pub fn calc_probs(v: &[Vec<i64>]) -> BTreeMap<i64, i64> {
    let mut cmap = BTreeMap::new();
    cmap.insert(0, 1);
    for d in v {
        let mut m2 = BTreeMap::new();
        for (k, v) in &cmap {
            for roll in d {
                let added = *k + *roll;
                let prev = *(m2.get(&added).unwrap_or(&0));
                m2.insert(added, prev + v);
            }
        }
        cmap = m2;
    }
    cmap
}

pub fn print_probabilities(b: &BTreeMap<i64, i64>) {
    let tot_p = b.values().fold(0, |n, v| n + v);

    println!("Tot options = {}", tot_p);
    for (k, v) in b {
        let pcent = 100. * (*v as f64) / (tot_p as f64);
        let mut at_least = 0;
        for (k2, v2) in b {
            if k2 >= k {
                at_least += v2
            }
        }
        let ls_pcent = 100. * (at_least as f64) / (tot_p as f64);
        println!(
            "ex {} : {}/{} = {:.2}\t\tmin {0} : {}/{2} = {:.2}",
            k, v, tot_p, pcent, at_least, ls_pcent
        );
    }
}
