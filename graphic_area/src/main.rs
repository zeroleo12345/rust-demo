mod graphic;
use graphic::{*};

fn print_area <T: AreaCalculable> (graphic: T) {
    let area = graphic.calculate_area();
    println!("This graphic area is: {area}");
}

fn main() {
    let circle = Circle{r: 3.0};
    print_area(circle);
    let triangle = Triangle{a: 3.0, h: 5.0};
    print_area(triangle);
    let square = Square{a: 3.0};
    print_area(square);
}
