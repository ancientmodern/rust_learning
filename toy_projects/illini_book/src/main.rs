mod illini_book;
use illini_book::IlliniBook;

fn run_illini_book(illini_book: IlliniBook) {
    illini_book.print_graph();
    println!("====================\nAreRelated");
    println!("{}", illini_book.are_related(1, 2));
    println!("{}", illini_book.are_related(3, 2));
    println!("{}", illini_book.are_related(1, 9));
    println!("{}", illini_book.are_related_relation(1, 2, "128"));
    println!("{}", illini_book.are_related_relation(1, 2, "124"));
    println!("{}", illini_book.are_related_relation(1, 6, "128"));
    println!("{}", illini_book.are_related_relation(1, 6, "124"));

    println!("====================\nGetRelated");
    println!("{}", illini_book.get_related(1, 2));
    println!("{}", illini_book.get_related(3, 2));
    println!("{}", illini_book.get_related(1, 9));
    println!("{}", illini_book.get_related_relation(1, 2, "128"));
    println!("{}", illini_book.get_related_relation(1, 2, "124"));
    println!("{}", illini_book.get_related_relation(1, 6, "128"));
    println!("{}", illini_book.get_related_relation(1, 6, "124"));

    println!("====================\nGetSteps");
    println!("{:?}", illini_book.get_steps(1, 1));
    println!("{:?}", illini_book.get_steps(1, 2));
    println!("{:?}", illini_book.get_steps(1, 3));
    println!("{:?}", illini_book.get_steps(9, 1));

    println!("====================\nCountGroups");
    println!("{}", illini_book.count_groups());
    println!("{}", illini_book.count_groups_relation("128"));
    println!("{}", illini_book.count_groups_relation("124"));
    println!("{}", illini_book.count_groups_relation("173"));
    println!("{}", illini_book.count_groups_relations(vec!["128", "173"]));
    println!(
        "{}",
        illini_book.count_groups_relations(vec!["128", "124", "173"])
    );
}

fn main() {
    let people_fpath = "example/persons.csv";
    let relations_fpath = "example/relations.csv";

    match IlliniBook::new(people_fpath, relations_fpath) {
        Ok(illini_book) => run_illini_book(illini_book),
        Err(err) => eprintln!("Error creating IlliniBook: {}", err),
    }
}
