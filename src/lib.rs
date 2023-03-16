pub trait Aggregators {

    fn max<T> (&self,list_of_items:&Vec<T>) -> &T;
    fn min<T> (&self, list_of_items:&Vec<T>) -> &T;
}
