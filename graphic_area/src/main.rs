mod graphic;
use graphic::{AreaCalculable, Circle};

fn print_area <T: AreaCalculable> (graphic: T) {
    let area = graphic.calculate_area();
    println!("area: {area}");
}

fn main() {
    let circle = Circle{r: 3.0};
    print_area(circle);
}
