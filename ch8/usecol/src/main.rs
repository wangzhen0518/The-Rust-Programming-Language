use std::collections::{HashMap, HashSet};

fn stat(arr: &Vec<isize>) -> (f64, isize, isize) {
    let mut mean = 0.0;
    let mut map = HashMap::new();
    for i in arr.iter() {
        mean += *i as f64;
        let cnt = map.entry(*i).or_insert(0);
        *cnt += 1;
    }
    mean /= arr.len() as f64;

    let mut arr = arr.clone();
    arr.sort_unstable();
    let mid = (arr.len() - 1) / 2;
    let mid = arr[mid];

    let mut cnt = 0;
    let mut most = arr[0];
    for (i, n) in map.iter() {
        if *n > cnt {
            cnt = *n;
            most = *i;
        }
    }

    (mean, mid, most)
}

fn to_pig_latin(s: &str) -> String {
    let vowel = HashSet::from(["a", "e", "i", "o", "u"]);
    let c = &s[..1];
    let other = &s[1..];
    if vowel.contains(c) {
        format!("{}-hay", s)
    } else {
        format!("{}-{}ay", other, c)
    }
}

struct Department {
    department: HashMap<String, Vec<String>>,
}

impl Department {
    fn add(&mut self, dm: &str, p: &str) {
        let p_list = self.department.entry(dm.to_string()).or_insert(Vec::new());
        p_list.push(p.to_string());
    }
    fn list_all(&self) -> Vec<String> {
        let mut p_list: Vec<String> = Vec::new();

        for p in self.department.values() {
            for pi in p {
                p_list.push(pi.clone())
            }
        }
        p_list.sort_unstable();
        p_list
    }
    fn list_department(&self, dm: &str) -> Option<&Vec<String>> {
        self.department.get(dm)
    }
}

fn main() {
    let mut d = Department {
        department: HashMap::new(),
    };
    d.add("abc", "cyl");
    d.add("hgj", "wangzhen");
    d.add("abc", "jk");

    let p_list = d.list_all();
    println!("{:#?}", p_list);

    match d.list_department("abc") {
        Some(p) => println!("{:#?}", p),
        None => println!("No such department"),
    }
    match d.list_department("adasd") {
        Some(p) => println!("{:#?}", p),
        None => println!("No such department"),
    }

    let x = to_pig_latin("asldkn");
    println!("{}", x);
    let x = to_pig_latin("bad");
    println!("{}", x);
}
