fn main() {
    // factorial
    //fn factorial(i: u64) -> u64 {
    //    fn fact_tr(i: u64, acc: u64) -> u64 {
    //        match i {
    //            0 => acc,
    //            n => fact_tr(n-1, n*acc)
    //        }
    //    }
    //    fact_tr(i, 1)
    //}
	//println!("{}", factorial(3)); 

    // immutability
    //let mut foo = "bar";
    //foo = "baz";
	//println!("{}", foo);
    
    // ownership
    //{
    //    let mut foo = "bar";
    //}
    //foo = "baz";
    //println!("{}", foo);

    // types
    //let i: i32 = 35;
    //let j: f32 = 3.0;
    //let heart_eyed_cat: char = '😻';
    //let konnichiwa = "こんにちは";
    //let nope: Option<i32> = None;
    //let maybe = Some(5);
    //let test = nope < maybe;
    //let v = vec![1, 2, 3];
    //println!("{} {}", test, konnichiwa);
    
    // structs
    //struct Rectangle {
    //    width: u32,
    //    height: u32,
    //}
    //impl Rectangle {
    //    fn area(&self) -> u32 {
    //        self.width * self.height
    //    }
    //}
    //let rect1 = Rectangle { width: 30, height: 50 };
    //println!("The area of the rectangle is {} square pixels.", rect1.area());

	// vectors
	//let mut vector: Vec<i32> = vec![1, 2, 3, 4];
	//vector.push(5);
	//println!("{}", &vector[4]);

	// closures
	//let id = |x| x;
	//println!("{} {}", id(5), id("foo"));

	// iterators
    //let v = vec![1, 2, 3];
	//let v_iter = v.iter().map(|x| x + 1);
	//let v_iter2 = v.iter().fold(0, |acc, &x| acc + x);
	//println!("{} {}", v_iter, v_iter2)

	println!("Just uncomment any lines you want to try and exectute :)");
}
