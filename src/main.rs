type Color = i32;

#[derive(Debug)]
struct Point {
	x: f64,
	y: f64,
	z: f64
}

impl std::ops::Add for Point {
	type Output = Point;

	fn add(self, other: Point) -> Point {
		Point {
			x: other.x + self.x,
			y: other.y + self.y,
			z: other.z + self.z
		}
	}
}

//Could do type Vector = Point, but then the two would compare equal, share methods, etc
struct Vector {
	x: f64,
	y: f64,
	z: f64
}

struct Sphere {
	center: Point,
	radius: f64
}

const SCREEN_WIDTH: usize = 300;
const SCREEN_HEIGHT: usize = 300;

const VIEW_WIDTH: f64 = 100 as f64;
const VIEW_HEIGHT: f64 = 100 as f64;

const FOCAL_POINT: Point = Point {
	x: 0 as f64,
	y: 0 as f64,
	z: 10 as f64
};

fn main() {
    let field = create_field();

    render(&field);

    println!("DONE")
}

fn create_field() -> Vec<Sphere> {
	let mut field = Vec::new();
	field.push(Sphere {
    	center: Point {
    		x: 0 as f64,
    		y: 0 as f64,
    		z: -10 as f64
    	},
    	radius: 5 as f64
    } );
	field
}

fn render(field: &Vec<Sphere>) -> Vec<Vec<Color>> {
	println!("Hello, field");
	for s in field {
		println!("Got a sphere at {:?} with radius {}", s.center, s.radius)
	}
	let mut rows = Vec::new();
	//Wish I knew how to check the length of one of these
	for y in 0..SCREEN_HEIGHT {
		let row = Vec::new();
		for x in 0..SCREEN_WIDTH {

		}
		rows.push(row)
	}
	rows
}

//fn view_point_for_screen_coords(screen_x: usize, screen_y: usize) -> Point {
//	let view_x = (screen_x / SCREEN_WIDTH) * (VIEW_WIDTH /2f64);
//	let view_y = (screen_y / SCREEN_HEIGHT) * (VIEW_HEIGHT/2f64);
//	Point {
//		x: view_x,
//		y: view_y,
//		z: 0
//	}
//}