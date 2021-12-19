struct Audio(String);
struct Video(String);


pub trait Playable {
    fn play(&self);
    fn pause() {
        println!("Paused");
    }
}

impl Playable for Audio {
    fn play(&self) {
        println!("Now playing audio: {} ", self.0);
    }
}

impl Playable for Video {

    fn play(&self) {
        println!("Now playing video: {} ", self.0);
    }
}

fn main() {
    println!("Super Player!");
    let audio = Audio("ambient_music.mp3".to_string());
    let video = Video("ambient_music.mp3".to_string());
    audio.play();
    video.play();
}
