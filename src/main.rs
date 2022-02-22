struct Class {

}

impl Class {
    fn new(_name: &str, _foo: &str, _counter: u32) -> Self {
        Self {
        }
    }

    pub fn validation(&self) {

    }
}
fn main() {
    for _ in 0..12 {
        let now = std::time::Instant::now();
        // ===============================================
        
        let mut v: Vec<Class> = Vec::with_capacity(100);
        for i in 0..100 {
            v.push(Class::new("fname", "fname", i % 30));
        }

        //for i in 0..100 {
        //    let cls = v.get(i).unwrap();
        //    cls.validation();
        //}
        //while let Some(top) = v.pop() {
        //    top.validation();
        //}
        for top in v.iter() {
            top.validation();
        }
        let now2 = std::time::Instant::now().duration_since(now).as_nanos();
        println!("==> {}", now2);
    }

}
