struct user{
    name:String,
    age:u32,
    gender:bool,
}
fn main() {
    println!("Hello, structure!");
    let user1 = user{
        name:String::from("jack"),
        age:20,
        gender:true,
    };
    println!("name {} \nage {} \ngender {}",user1.name, user1.age, 
             if user1.gender{
                 String::from("male")
             }
             else{
                 String::from("female")
             });
}
