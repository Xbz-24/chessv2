pub struct Position{
    pub x: i32,
    pub y: i32,
}
impl Position{
    pub fn new(x: i32, y: i32)->Self{
        Self{x, y}
    }
    // you might want to implement additional methods to manipulate or compare positions, for example
    // compute the manhattan distance between two positions
    pub fn distance_for(&self, other: &Self)->i32{
        (self.x - other.x).abs()+ (self.y - other.y).abs()
    }
    // check if the position is within the board
    pub fn is_valid(&self)->bool{
        self.x >=0&&self.x<8&&self.y>=0&&self.y<8
    }
}