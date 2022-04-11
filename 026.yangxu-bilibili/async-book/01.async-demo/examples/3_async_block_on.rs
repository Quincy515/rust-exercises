use futures::executor::block_on;

struct Song{}

async fn learn_song() -> Song {
    Song{}
}

async fn sing_song(song: Song) {

}

async fn dance() {}


fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}