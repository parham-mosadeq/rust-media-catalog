#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

fn print_media(media: Media) {
    println!("{:#?}", media);
}

fn main() {
    let book = Media::Book {
        title: "TestBookTitle".to_string(),
        author: "testUsers".to_string(),
    };

    let movie = Media::Movie {
        title: String::from("good fellas"),
        director: String::from("Martin"),
    };

    let audio_book = Media::Audiobook {
        title: String::from("someRandomAudioBook"),
    };

    print_media(book);
    print_media(movie);
    print_media(audio_book);
}
