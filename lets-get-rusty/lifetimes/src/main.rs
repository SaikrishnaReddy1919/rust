fn main() {
    println!("Hello, world!");

    let s1 = String::from("sai");
    let res;
    {
        let s2 = String::from("krishna");
        res = longest(&s1,&s2);
        println!("Longest is : {}", res);
    }


    // <-----CASE2----->
    let s1 = String::from("sai");
    let res;
    {
        let s2 = String::from("krishna");
        // not works because res lifetime ends when s2 lifetime ends
        // this is because longest considers smallest lifetime -> here it is s2
        res = longest(&s1,&s2);
    }
    println!("Longest is : {}", res);

    // <-----CASE3----->

    let s1 = String::from("sai");
    let res;
    {
        let s2 = String::from("krishna");
        //works because longest_X considers x
        res = longest_x(&s1,&s2);
    }
    println!("Longest is : {}", res);

}

//considers smallest lifetime 
fn longest<'a>(x : &'a str, y : &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//doesnt matter lifetime of y, considers x lifetime
fn longest_x<'a>(x : &'a str, y : &str) -> &'a str {
    x
}