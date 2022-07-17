use std::{time::{SystemTime, Duration}, thread::sleep};



fn processInput(){

}
fn update(){

}
fn render(){

}
fn main() {
    let frames = Duration::new(0,5000000);
    let gameover = true;
    while gameover {
        let start = SystemTime::now();
        processInput();
        update();
        render();
        let dur = match start.elapsed() {
            Ok(dur) => dur,
            Err(e) => panic!("{}", e)
        };
        sleep(dur+frames);
    }
}
