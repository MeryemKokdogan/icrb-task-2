fn main(){
    let collection = vec![234,56,73,4267,878,8,95,34,123,58,789];

    let filter = FilterCondition{filter:100};

    let filtered_collection = custom_filter(collection, &filter);

    println!("{:?}", filtered_collection);

}
struct FilterCondition<T>{
    filter:T,
}

impl <T:PartialOrd> FilterCondition<T>{
    fn is_match(&self, item: &T) -> bool {
         item > &self.filter
    }

} 

fn custom_filter<T> (collection:Vec<T>, condition:&FilterCondition<T>) -> Vec<T> where T:PartialOrd{
         
         let mut new_collection = Vec::new();

    for item in collection{
        if condition.is_match(&item){
            new_collection.push(item)
        }
    }
    new_collection
   
}
