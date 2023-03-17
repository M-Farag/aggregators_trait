use aggregator::Aggregators;

#[derive(Debug)]
struct Numbers {
    list_of_numbers:Vec<usize>,
}

impl Aggregators for Numbers{
    fn new_list() -> Self
    {
        Self { list_of_numbers: Vec::new() }
    }

    fn min(&self) ->&usize {
        let mut min = self.list_of_numbers.get(0).unwrap();
        for num in &self.list_of_numbers {
            if num < min {
                min = num;
            }
        }
        min
    }

    fn max(&self) ->&usize {
        let mut max = self.list_of_numbers.get(0).unwrap();
        for num in &self.list_of_numbers {
            if num > max {
                max = num;
            }
        }
        max
    }
}

fn main() {
    let mut x:Numbers = Numbers::new_list();
    x.list_of_numbers.push(1);
    x.list_of_numbers.push(6);
    x.list_of_numbers.push(3);

    println!("Min value is: {}",x.min());
    println!("Max value is: {}",x.max());

    println!("Greetings with a default implementation: {}", x.greetings());
}

