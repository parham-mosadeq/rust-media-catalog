#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    // * Unlabeled fields
    Podcast(u32),
    Placeholder,
}

fn print_media(media: &Media) {
    println!("{:#?}", media);
}

impl Media {
    fn description(&self) -> String {
        // if let Media::Book { title, author } = self {
        //     format!("Book desc is: {title} and {author}")
        // } else if let Media::Movie { title, director } = self {
        //     format!("Movie desc is: {title} and {director}")
        // } else if let Media::Audiobook { title } = self {
        //     format!("Audiobook desc is: {title}  ")
        // } else {
        //     String::from("Media Desc")
        // }

        // * Two ways to write match
        match self {
            Media::Book { title, author } => format!("Book desc is: {title} and {author}"),
            Media::Audiobook { title } => {
                format!("Audiobook desc is: {title}  ")
            }
            Media::Movie { title, director } => {
                format!("Movie desc is: {title} and {director}")
            }
            // * Can use anything (id,number,episode_number...)
            Media::Podcast(id) => {
                format!("Podcast desc is: {id} ")
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}
#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![] }
    }

    fn add(&mut self, media: Media) {
        self.items.push(media);
    }
}

fn main() {
    let mut catalog = Catalog::new();

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

    println!("{}", audio_book.description());
    println!("{}", movie.description());
    println!("{}", book.description());
    print_media(&book);
    print_media(&movie);
    print_media(&audio_book);
    catalog.add(audio_book);
    catalog.add(book);
    catalog.add(movie);
    println!("{:#?}", catalog);
}
