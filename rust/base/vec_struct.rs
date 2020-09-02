#[derive(Debug)]

struct Person {
    name: &'static str,
    age: u32,
}

impl Person {
    fn greet(&self) {
        println!("Hello, my name is {} and age is {}", self.name, self.age);
    }

    fn tellage(&self){
        println!("The age is {}",self.age);
    }
}

impl Default for Person {
    fn default()->Self{
        Self {name:"A", age:20,}        
    }
}

fn main() {
    let person = Person { name: "Herman", age: 32, };
    let k = Person{name:"hey", age:64,};
    let mut  v1: Vec<Person> =Vec::new();
    let m=Person::default();
    v1.push(person);
    v1.push(k);
    v1.push(m);
    for k in v1.iter(  ){
        k.greet();
    }
}
