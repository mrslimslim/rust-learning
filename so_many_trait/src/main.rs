struct Engineer {
}
struct ProductManger {
}

trait Staff {
    fn work(&self) -> String;
}

impl Staff for Engineer {
    fn work(&self) -> String{
        let str = String::from("My job is writing code");
        str
    }
}

impl Staff for ProductManger {
    fn work(&self) -> String {
        String::from("My job is make better product")
    }
}

fn choose_better_staff() -> Box<dyn Staff> {
    if(1 > 0 ){
        Box::new(Engineer {})
    }else{
        Box::new(ProductManger {})
    }
   
}

fn main() {
   let kiko = Engineer {
   };
   let shine = ProductManger {
   };

    
}
