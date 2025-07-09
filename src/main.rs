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

    print_media(book);
}
