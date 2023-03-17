pub trait Aggregators {
    fn max(&self) -> &usize;
    fn min(&self) -> &usize;
    fn new_list() -> Self;
    
    fn greetings(&self) -> String
    {
        String::from("Hello Munchy")
    }

    
}
