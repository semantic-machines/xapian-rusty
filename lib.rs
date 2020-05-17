const BRASS: i8 = 1;
const CHERT: i8 = 2;
const IN_MEMORY: i8 = 3;

/** Open for read/write; create if no db exists. */
const DB_CREATE_OR_OPEN: i8 = 1;
/** Create a new database; fail if db exists. */
const DB_CREATE: i8 = 2;
/** Overwrite existing db; create if none exists. */
const DB_CREATE_OR_OVERWRITE: i8 = 3;

#[cxx::bridge(namespace = org::example)]
mod xapian {
    pub(crate) struct SharedThing {
        pub(crate) z: i32,
        pub(crate) y: Box<ThingR>,
        x: UniquePtr<ThingC>,
    }

    extern "C" {
        include!("xapian-bind.h");

        type ThingC;
        pub(crate) fn make_demo(appname: &str) -> UniquePtr<ThingC>;
        pub(crate) fn get_name(thing: &ThingC) -> &CxxString;
        pub(crate) fn do_thing(state: SharedThing);

        type Database;
        pub(crate) fn new_database(err: &mut i8) -> UniquePtr<Database>;
        pub(crate) fn new_database_with_path(path: &str, db_type: i8, err: &mut i8) -> UniquePtr<Database>;
        pub(crate) fn database_reopen(db: &mut Database, err: &mut i8);

        type Stem;
        pub(crate) fn new_stem(lang: &str, err: &mut i8) -> UniquePtr<Stem>;

        type WritableDatabase;
        pub(crate) fn new_writable_database_with_path(path: &str, action: i8, db_type: i8, err: &mut i8) -> UniquePtr<WritableDatabase>;
        pub(crate) fn commit(db: &mut WritableDatabase, err: &mut i8);
        pub(crate) fn replace_document(db: &mut WritableDatabase, unique_term: &str, doc: &mut Document, err: &mut i8) -> u32;
        pub(crate) fn delete_document(db: &mut WritableDatabase, unique_term: &str, doc: &mut Document, err: &mut i8);

        type TermGenerator;
        pub(crate) fn new_termgenerator(err: &mut i8) -> UniquePtr<TermGenerator>;
        pub(crate) fn set_stemmer(tg: &mut TermGenerator, stem: &mut Stem, err: &mut i8);
        pub(crate) fn set_document(tg: &mut TermGenerator, doc: &mut Document, err: &mut i8);
        pub(crate) fn index_text(tg: &mut TermGenerator, data: &str, err: &mut i8);
        pub(crate) fn index_int(tg: &mut TermGenerator, data: i32, err: &mut i8);
        pub(crate) fn index_long(tg: &mut TermGenerator, data: i64, err: &mut i8);
        pub(crate) fn index_float(tg: &mut TermGenerator, data: f32, err: &mut i8);
        pub(crate) fn index_double(tg: &mut TermGenerator, data: f64, err: &mut i8);

        type Document;
        pub(crate) fn new_document(err: &mut i8) -> UniquePtr<Document>;
        pub(crate) fn add_string(doc: &mut Document, slot: u32, data: &str, err: &mut i8);
        pub(crate) fn add_int(doc: &mut Document, slot: u32, data: i32, err: &mut i8);
        pub(crate) fn set_data(doc: &mut Document, data: &str, err: &mut i8);
    }

    extern "Rust" {
        type ThingR;
        fn print_r(r: &ThingR);
    }
}

pub struct ThingR(usize);

fn print_r(r: &ThingR) {
    println!("called back with r={}", r.0);
}
/*
fn main() {
    unsafe {
        test();
    }
}

unsafe fn test() {
    let x = xapian::make_demo("xapian of cxx::bridge");
    println!("this is a {}", xapian::get_name(x.as_ref().unwrap()));

    xapian::do_thing(xapian::SharedThing {
        z: 222,
        y: Box::new(ThingR(335)),
        x,
    });

    let mut err: i8 = 0;

    let mut db1 = xapian::new_database_with_path("./xapian-search-base", CHERT, &mut err);
    println!("open db, err={}", err);

    if err >= 0 {
        xapian::database_reopen(&mut db1, &mut err);
        println!("reopen db, err={}", err);
    }

    let mut stem = xapian::new_stem("russian", &mut err);
    println!("new stem, err={}", err);

    let mut db_w = xapian::new_writable_database_with_path("./w1", DB_CREATE_OR_OPEN, CHERT, &mut err);
    println!("open rw db, err={}", err);

    let mut tg = xapian::new_termgenerator(&mut err);
    println!("new term generator db, err={}", err);

    xapian::set_stemmer(&mut tg, &mut stem, &mut err);
    println!("set stem, err={}", err);

    let mut doc = xapian::new_document(&mut err);
    println!("new document, err={}", err);

    let id = "1234567kazjkhgkjazhwetyiwyeiu";

    xapian::set_data(&mut doc, id, &mut err);
    println!("set data, err={}", err);

    xapian::add_string(&mut doc, 1, "TEST TEST ТЕСТ aslfkgskj aszgksndgj aklglkajlkjlk", &mut err);
    println!("add string, err={}", err);

    xapian::add_int(&mut doc, 2, 9999999, &mut err);
    println!("add int, err={}", err);

    xapian::set_document(&mut tg, &mut doc, &mut err);
    println!("set_document, err={}", err);

    xapian::replace_document(&mut db_w, id, &mut doc, &mut err);
    println!("replace_document, err={}", err);

    xapian::commit(&mut db_w, &mut err);
    println!("commit, err={}", err);
}
*/