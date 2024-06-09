#[derive(Debug)]
enum Directions{
    North,
    East,
    South,
    West
}
fn main(){
    let _my_direction = Directions::East;
    move_around(_my_direction)
}
fn move_around(direction:Directions){
    println!("Moving {:?}", direction   );
}