fn main() {
    //generics helps reducing code
    let num_list = vec![1, 2, 3, 4, 5];

    let largest = get_largest(num_list);

    // above function works for any list of type vec<i32> but what if we have char list ?
    // we can have one more function to compare chars in a list returns the largest char but how can we do it with single function?
    // we use generics.

    let char_list = vec!['k', 'r', 'i', 's', 'h', 'n', 'a'];

    let char_lar = get_largest(char_list);
    
    println!("Largest num is : {} and Largest char is : {}", largest, char_lar);
    generics_in_structs_and_structs::test_square();

}

// <--------Without generics---cant use below function for a list other tham i32 list----->
// fn get_largest(num_list : Vec<i32>) -> i32 {
//     let mut largest =num_list[0];
//     for num in num_list {
//         if num > largest {
//             largest = num
//         }
//     }
//     largest
// }

// below function can be used for i32 list and char list
fn get_largest<T : PartialOrd + Copy>(num_list : Vec<T>) -> T {
    let mut largest =num_list[0];
    for num in num_list {
        if num > largest {
            largest = num
        }
    }
    largest
}



mod generics_in_structs_and_structs {
    // x and y are of same type T
    struct Point<T> {
        x : T,
        y : T
    }

    struct Point2<T, U>{
        x : T,
        y : U
    }


    fn test(){
        let x1 = Point { x : 4, y : 5};
        // x2 will not be possible if struct is : 
        #[allow(unused_doc_comments)]
        /**
         * struct Point {
         *      x : i32,
         *      y : i32
         * }
         */
        // with Point being of generic type we can have any type of point
        let x2 = Point { x : 10.9, y : 8.9};


        let x3 = Point2 { x : 32, y : 5.9};
    }


    // generics in enums
    enum Option<T>{
        Some(T),
        None
    }

    enum Result<T, E>{
        Ok(T),
        Err(E)
    }

    // examples
    struct Circle<T> {
        x : T,
        y : T
    }

    //function x is avaibale on any type of circle
    impl<U> Circle<U> {
        fn X(&self) -> &U {
            &self.x
        }
    }
    // function y is only availble on cricle of type f64
    impl Circle<f64> {
        fn y(&self) -> f64 {
            self.y
        }
    }

    fn test_above(){
        let x = Circle {x : 10, y : 10};
        // x.y(); -> not available here becuase x and y are i32
        x.X();

        let y = Circle { x : 10.5, y : 5.6};
        y.y(); // -> caz x and y are floars
        y.X(); // -> X() available on both caz type if <U>
    }

    // advance

    struct Square<T, U> {
        x : T,
        y : U
    }

    impl <T, U> Square<T, U> {
        fn mixup<V, W>(self, other : Square<V, W>) -> Square<T, W>{
            Square {
                x : self.x, 
                y : other.y,
            }
        }
    }

    pub fn test_square(){
        let s1 = Square { x : 10, y : 10.5};
        let s2 = Square { x : "Hai", y : 'c'};

        let s3 = s1.mixup(s2);
        println!("s3.x is  : {} and s3.y is : {}", s3.x, s3.y);
    }


    // generics helps in reducing the code

    enum Option1<T> {
        Some(T),
        None
    }

    fn test_option() { 
        // above code works for int and float...we done have declare another enum to accept float.
        // when code compiles, rust creates another enum. one for int and onw for float. This does not incur performance issue
        //Then above code looks like below :
        #[allow(unused_doc_comments)]
        /**
         * enum Option<i32> {
         *      Some(i32),
         *      None
         * }
         * 
         * enum Option<f64> {
         *      Some(f64),
         *      None
         * }
         */
        let integer = Option::Some(10);
        let float = Option::Some(10.5);
    }

    
}

