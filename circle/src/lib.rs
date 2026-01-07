#[derive(Debug, Clone, Copy)]
pub struct Circle {
	pub center: Point,
	pub radius: f64,
}

impl Circle {
fn new(x: f64, y:f64, radius: f64)-> Circle {
    Circle{
        center: Point(x, y),
        radius: radius
    }
}
 
 fn area(&self) -> f64{
     let pi = 3.14159;
     pi*(self.radius * self.radius)
 }   
    
    fn diameter(&self) -> f64{
        2.0*self.radius
    }
    fn intersect(&self, other: Circle) ->bool{
        let d = self.center.distance(other.center);
        self.radius - other.radius < d && d < self.radius - other.radius  
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(f64, f64);

impl Point {
fn distance(&self, other: Point)->f64{
    let dx = other.0 - self.0;
    let dy = other.1 - self.1;
    (dx*dx + dy *dy).sqrt()
}
}