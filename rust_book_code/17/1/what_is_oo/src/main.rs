pub struct AveragedCollection {
    list : Vec<i32>,
    average:f64,
}

impl AveragedCollection {
    fn new(int:i32) -> AveragedCollection{
        let average = int;
        let mut list:Vec<i32> = Vec::new();
        list.push(average);

        AveragedCollection{
            list,
            average: average as f64,
        }

    }

    pub fn add(&mut self ,value:i32){
        self.list.push(value);
        self.update_average();
    }
    
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) =>{
                self.update_average();
                Some(value)
            },
            None => None,
        }
    }

    pub fn average(&self) -> f64{
        self.average
    }

    fn update_average(&mut self) {
        let total:i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


fn main() {
    let mut averaged_collection = AveragedCollection::new(3);
    averaged_collection.add(5);
    averaged_collection.add(7);
    println!("{}",averaged_collection.average());
    
}
