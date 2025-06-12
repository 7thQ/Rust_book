fn main() {
    println!("Hello, world!");

    let x = another_fn(100);

    another_fn(200);

    another_unit(46);


    println!("{x}");

}


fn another_fn (x: i64 ) -> i64 {

    println!("{x}");
    x + 5

    

}


fn another_unit (x: i32)  {

    println!(" Unit value is a type, thats used when you dont care/need evalute to or return a resultant \n thats why i can pass in a value and preform some arithmitic & add a semicolon to the end of it after this print statement.\n  As long a the reutrn value is not needed else where then i should be good or the definition of the fn isnt specifiying a return type ");

    x + 4;


}