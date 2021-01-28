use std::cmp;
use std::collections::HashMap;
use std::hash;

struct Cacher<T: Fn(&TArg) -> &TResult, TArg: cmp::Eq + hash::Hash + Clone, TResult: Clone> {
    cache: HashMap<TArg, TResult>,
    calculation: T,
}

impl<T: Fn(&TArg) -> &TResult, TArg: cmp::Eq + hash::Hash + Clone, TResult: Clone>
    Cacher<T, TArg, TResult>
{
    fn new(calculation: T) -> Cacher<T, TArg, TResult> {
        Cacher {
            cache: HashMap::new(),
            calculation,
        }
    }

    fn value(&mut self, arg: &TArg) -> TResult {
        match self.cache.get(arg) {
            Some(v) => v.clone(),
            None => {
                let v = (self.calculation)(arg).clone();
                self.cache.insert(arg.clone(), v.clone());
                v
            }
        }
    }
}

fn main() {
    let mut cacher = Cacher::new(|x| x);

    println!("{}", cacher.value(&String::from("hello")));
    println!("{}", cacher.value(&String::from("hello")));
    println!("{}", cacher.value(&String::from("what")));
}
