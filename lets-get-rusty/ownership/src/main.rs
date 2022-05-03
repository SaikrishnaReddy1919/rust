fn main() {
    // ------------ Ownership Rules -------------
    // 1. Each value in rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped


    let x : i32 = 5;
    let y : i32 = x; // types(integers, chars, floats) stored on stack will be copied.


    let s1 = String::from("Hello") ;
    // let s2 = s1; -> Wrong, types stored on heap will be moved....here "hello" will be moved to s2.
    let s2 = s1.clone(); // clone the value if you want a copy of value stored on heap.f 


    println!("x : {}, y: {}, s1 : {}, s2 : {}", x, y, s1, s2);


    /*
     In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, 
     and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is 
     no longer being used and call code to explicitly return it, just as we did to request it. Doing this correctly has 
     historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, 
     we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.
     */
}
