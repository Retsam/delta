trait FieldObject {
	
}

#[derive(Debug)]
struct Point {
	x: i32,
	y: i32,
	z: i32
}

struct Sphere {
	center: Point,
	radius: i32
}

fn main() {
    let field = create_field();

    render(&field);
}

fn create_field() -> Vec<Sphere> {
	let mut field = Vec::new();
	field.push(Sphere {
    	center: Point {
    		x: 0,
    		y: 0,
    		z: -10
    	},
    	radius: 5
    } );
	field
}

fn render(field: &Vec<Sphere>) {
	println!("Hello, field");
	for s in field {
		println!("Got a sphere at {:?} with radius {}", s.center, s.radius)
	}
}