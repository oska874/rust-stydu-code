

fn add2(a:i32, b:i32) -> i32 
{
    a+b
}

fn printstr(str:String)
{
    println!("111 {}",str);
}

fn printstr2(str:String) -> String
{
    println!("222 {}",str);
    str
}

fn printstr3(str:&String)
{
    println!("333 {}",str);
}

fn printstr4(str: &mut String)
{
    println!("444 {}",str);
    str.push_str(" 456");
}

fn printstr5(strs:&String) -> &str
{
    println!("555 {}",strs);
    &strs[0..=3]
}
    

fn main() {
    println!("Hello, world!");
    let x1=2;
    let x2=3;
    let x = add2(x1,x2);
    println!("add sum {}",x);
    let mut st1 = String::from("123");
    st1 = printstr2(st1);
    printstr3(&st1);
    printstr4(&mut st1);
    let st2 = printstr5(&st1);
    printstr(st2.to_string());
    st1.clear();
    println!("{}",st1);
    let st3 = &st1[4..6];


    let ary1 = [1,2,3];
    println!("{}",ary1[0]);

    let tup1 = (1,1.2,"xxx");
    println!("{} {}",tup1.2, tup1.1);
}
