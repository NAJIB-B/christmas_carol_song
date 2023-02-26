fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth"];
    let gifts = ["Two turtle doves","Three French hens", "Four calling birds",  "Five goldenen rings",
    "Six geese a-laying", "Seven swans a-swimming", "Eight maids a-milking", "Nine ladies dancing"];
    let mut count:usize = 0;
    let mut inner_count:usize = 0;
    for element in days{
        println!("on the {} day of christmas", element);
        println!("My true love brought to me");
        if count > 0{
         while inner_count > 0{
            println!("{}", gifts[inner_count -1]);
            inner_count-= 1;
         };
         println!("And a partridge in a pear tree");
         println!("\n");
        };
        if count == 0{

        println!("a partridge in a pear tree");
        println!("\n");
        }
               
        count+= 1;
        inner_count = count; 
    }
}
