use csv;
use serde::Serialize;
use std::io;
fn main() {
    let mut wrt = csv::Writer::from_writer(io::stdout());
    wrt.write_record(&["Name", "Place", "ID"]).unwrap();
    wrt.serialize(("Mark", "Sydney", 87)).unwrap();
    wrt.flush();

    /***********************
     * SERIALIZE USING SERDE
     ***********************/
    #[derive(Serialize)]
    struct Book<'a> {
        title: &'a str,
        author: &'a str,
        genre: &'a str,
        height: u32,
        publisher: &'a str,
    }

    /*
        "Title,Author,Genre,Height,Publisher
    Fundamentals of Wavelets,Goswami, Jaideva,signal_processing,228,Wiley
    Data Smart,Foreman, John,data_science,235,Wiley
    God Created the Integers,Hawking, Stephen,mathematics,197,Penguin
    Superfreakonomics,Dubner, Stephen,economics,179,HarperCollins
    Orientalism,Said, Edward,history,197,Penguin
    Nature of Statistical Learning Theory, The,Vapnik, Vladimir,data_science,230,Springer
    Integration of the Indian States,Menon, V P,history,217,Orient Blackswan"
         */
    let book_1 = Book {
        title: "Fundamentals of Wavelets",
        author: "Jaideva Goswami",
        genre: "signal_processing",
        height: 228,
        publisher: "Wiley",
    };
    let book_2 = Book {
        title: "Nature of Statistical Learning Theory",
        author: "Vladimir Vapnik",
        genre: "data_science",
        height: 230,
        publisher: "Springer",
    };

    wrt.serialize(book_1);
    wrt.serialize(book_2);

    println!(" ");
    wrt.flush().unwrap();
}
