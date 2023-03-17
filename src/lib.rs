pub trait Aggregators {

    fn max<T: std::cmp::PartialOrd> (&self,list_of_items:&Vec<T>) -> &T;
    fn min<T: std::cmp::PartialOrd> (&self, list_of_items:&Vec<T>) -> &T;
}
