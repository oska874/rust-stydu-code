struct user{
    name:String,
    age:u32,
    gender:bool,
}

#[derive(Debug)]
struct rect{
    width:u32,
    high:u32,
}

impl rect{
    fn square_fn(size:u32) -> rect
    {
        rect{width:size,high:size}
    }
}


struct square{
    len:u32,
}

impl square{
    fn area(&self) -> u32
    {
        self.len*self.len
    }
    fn is_bigger(&self, other:&square) -> bool
    {
        if self.area() > other.area(){
            true
        }
        else{
            false
        }
    }
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
    println!("test2");

    let name = String::from("marry");
    let age=11;
    let user2=user {
        name,
        age,
        gender:false,
    };
    println!("name {} \nage {} \ngender {}",user2.name, user2.age, 
             if user2.gender{
                 String::from("male")
             }
             else{
                 String::from("female")
             });
    let user3 = user{
        name:user1.name,
        age:user2.age,
        gender:user2.gender,
    };
    println!("name {} \nage {} \ngender {}",user3.name, user3.age, 
             if user3.gender{
                 String::from("male")
             }
             else{
                 String::from("female")
             });
    let user40 = user{
        name:String::from("john"),
        age:33,
        gender:true,
    };
    let user4 = user{
        ..user40
    };
    println!("user4 name {} \nage {} \ngender {}",user4.name, user4.age, 
             if user4.gender{
                 String::from("male")
             }
             else{
                 String::from("female")
             });

    println!("test3");

    struct points(i32,i32);
    let x11=points(1,1);
    println!("{}.{}",x11.0,x11.1);

    println!("test4");

    let r1 = rect{
        width:10,
        high:12,
    };
    println!("{}",cal_square(r1));
    println!("test5");

    let sq1 = square{len:33};
    println!("{}",sq1.area());
    let sq2 = square{len:22};
    println!("{}", sq1.is_bigger(&sq2));
    println!("{:#?}", rect::square_fn(10));

}


fn cal_square(dim:rect)->u32
{
    println!("{:?}",dim);
    println!("{:#?}",dim);
    dim.width*dim.high

}
