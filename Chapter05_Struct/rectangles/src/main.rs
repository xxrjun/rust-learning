#[derive(Debug)] // 使 Rectangle 實作 Debug 功能
struct Rectangle {
    width: u32,
    height: u32,
}

/*
    All functions defined within an `impl` block are called `associated functions`
*/
impl Rectangle {
    /*
        This associated function does not need `&self`, so it's not a methods.
        And it is used to generate new struct instance, so called 'Constructor'.
    */
    fn build_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // Refractor with tuples
    let rec1 = (40, 20);
    println!(
        "The area of the rectangle1 is {} square pixels.",
        area_with_tuple(rec1),
    );

    let rec2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle2 is {} square pixels.",
        rec2.area(),
    );
    println!();

    /*
        對結構體來說 println! 要怎麼格式化輸出結果就會有點不明確了，因為顯示的方式就很有多種。 所以結構體預設並沒有 Display 的實作，也就無法使用 println! 與 {} 佔位符。
    */

    println!("Using debug and std output");
    println!("rect2 is {:#?}", rec2);
    println!();

    println!("Using !dbg");
    dbg!(&rec2);
    println!();

    // Automatic referencing and dereferencing 自動引用與解引用
    if rec2.width() {
        println!("rec2's width is not 0");
        println!();
    }

    let rec3 = Rectangle::build_square(16);

    println!("rec3 can hold rec2? {}", rec3.can_hold(&rec2));
    println!();
}

fn area_with_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
