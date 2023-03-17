use aggregator::Aggregators;
pub struct Element <T> {
    components:Vec<T>
}
impl<X> Aggregators for Element<X> {
    fn max<X: std::cmp::PartialOrd> (&self,list_of_items:&Vec<X>) -> &X {
        let mut max = list_of_items.get(0).unwrap();
        for item in list_of_items {
            if item > max {
                max = item;
            }
        }
        max
    }

    fn min<X: std::cmp::PartialOrd> (&self,list_of_items:&Vec<X>) -> &X {
        let mut min = list_of_items.get(0).unwrap();
        for item in list_of_items {
            if item < min {
                min = item;
            }
        }
        min
    }
}
