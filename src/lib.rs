pub const BRASS: i8 = 1;
pub const CHERT: i8 = 2;
pub const IN_MEMORY: i8 = 3;

/** Open for read/write; create if no db exists. */
pub const DB_CREATE_OR_OPEN: i8 = 1;
/** Create a new database; fail if db exists. */
pub const DB_CREATE: i8 = 2;
/** Overwrite existing db; create if none exists. */
pub const DB_CREATE_OR_OVERWRITE: i8 = 3;

#[cxx::bridge(namespace = org::example)]
pub mod xapian {
    extern "C" {
        include!("xapian-bind.h");

        type Database;
        pub fn new_database(err: &mut i8) -> UniquePtr<Database>;
        pub fn new_database_with_path(path: &str, db_type: i8, err: &mut i8) -> UniquePtr<Database>;
        pub fn database_reopen(db: &mut Database, err: &mut i8);

        type Stem;
        pub fn new_stem(lang: &str, err: &mut i8) -> UniquePtr<Stem>;

        type WritableDatabase;
        pub fn new_writable_database_with_path(path: &str, action: i8, db_type: i8, err: &mut i8) -> UniquePtr<WritableDatabase>;
        pub fn commit(db: &mut WritableDatabase, err: &mut i8);
        pub fn replace_document(db: &mut WritableDatabase, unique_term: &str, doc: &mut Document, err: &mut i8) -> u32;
        pub fn delete_document(db: &mut WritableDatabase, unique_term: &str, doc: &mut Document, err: &mut i8);

        type TermGenerator;
        pub fn new_termgenerator(err: &mut i8) -> UniquePtr<TermGenerator>;
        pub fn set_stemmer(tg: &mut TermGenerator, stem: &mut Stem, err: &mut i8);
        pub fn set_document(tg: &mut TermGenerator, doc: &mut Document, err: &mut i8);
        pub fn index_text(tg: &mut TermGenerator, data: &str, err: &mut i8);
        pub fn index_int(tg: &mut TermGenerator, data: i32, err: &mut i8);
        pub fn index_long(tg: &mut TermGenerator, data: i64, err: &mut i8);
        pub fn index_float(tg: &mut TermGenerator, data: f32, err: &mut i8);
        pub fn index_double(tg: &mut TermGenerator, data: f64, err: &mut i8);

        type Document;
        pub fn new_document(err: &mut i8) -> UniquePtr<Document>;
        pub fn add_string(doc: &mut Document, slot: u32, data: &str, err: &mut i8);
        pub fn add_int(doc: &mut Document, slot: u32, data: i32, err: &mut i8);
        pub fn set_data(doc: &mut Document, data: &str, err: &mut i8);
    }
}
