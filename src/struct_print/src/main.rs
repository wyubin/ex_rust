use std::fmt;
#[derive(Debug)]
struct MinMax(i64,i64);

struct Print2d{
	x:f64,
	y:f64
}

impl fmt::Display for MinMax {
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"({},{})",self.0,self.1)
	}
}
impl fmt::Display for Print2d {
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
		write!(f,"x: {}, y: {}",self.x,self.y)
	}
}
struct List(Vec<i32>);
impl fmt::Display for List {
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
		let tvec = &self.0;
		// first write
		write!(f,"[")?;
		for (tind,v) in tvec.iter().enumerate(){
			if tind != 0 { write!(f, ", ")?; }
			write!(f, "{}", v)?;
		}
		// final write
		write!(f, "]")
	}
}
struct City {
	name: &'static str,
	lat: f32,
	lon: f32,
}
impl fmt::Display for City {
	fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
		let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
		let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
		write!(f, "{}: {:.3}°{} {:.3}°{}",self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
	}
}
fn main() {
	println!("print of `minMax(3,6)`:{}",MinMax(3,6));
	println!("print by display of `minMax(3,6)`:{:?}",MinMax(3,6));
	println!("print by display of `print2d`:{}",Print2d{x:3.2,y:6.5});
	println!("print of `List(vec![1,2])`:{}",List(vec![1,2]));
	println!("print of `City(Taipei,53.1122,-23.1155)`:{}",City{name:"Taipei", lat:53.1122,lon:-23.1155});
}
