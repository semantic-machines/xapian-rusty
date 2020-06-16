use cxx::UniquePtr;

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
pub(crate) mod ffi {

    extern "C" {
        include!("xapian-bind.h");

        pub(crate) type Database;
        pub(crate) fn new_database(err: &mut i8) -> UniquePtr<Database>;
        pub(crate) fn new_database_with_path(path: &str, db_type: i8, err: &mut i8) -> UniquePtr<Database>;
        pub(crate) fn database_reopen(db: &mut Database, err: &mut i8);
        pub(crate) fn database_close(db: &mut Database, err: &mut i8);
        pub(crate) fn new_enquire(db: &mut Database, err: &mut i8) -> UniquePtr<Enquire>;
        pub(crate) fn add_database(db: &mut Database, add_db: &mut Database, err: &mut i8);

        pub(crate) type Stem;
        pub(crate) fn new_stem(lang: &str, err: &mut i8) -> UniquePtr<Stem>;

        pub(crate) type WritableDatabase;
        pub(crate) fn new_writable_database_with_path(path: &str, action: i8, db_type: i8, err: &mut i8) -> UniquePtr<WritableDatabase>;
        pub(crate) fn commit(db: &mut WritableDatabase, err: &mut i8);
        pub(crate) fn replace_document(db: &mut WritableDatabase, unique_term: &str, doc: &mut Document, err: &mut i8) -> u32;
        pub(crate) fn delete_document(db: &mut WritableDatabase, unique_term: &str, err: &mut i8);

        pub(crate) type TermGenerator;
        pub(crate) fn new_termgenerator(err: &mut i8) -> UniquePtr<TermGenerator>;
        pub(crate) fn set_stemmer(tg: &mut TermGenerator, stem: &mut Stem, err: &mut i8);
        pub(crate) fn set_document(tg: &mut TermGenerator, doc: &mut Document, err: &mut i8);
        pub(crate) fn index_text_with_prefix(tg: &mut TermGenerator, data: &str, prefix: &str, err: &mut i8);
        pub(crate) fn index_text(tg: &mut TermGenerator, data: &str, err: &mut i8);
        pub(crate) fn index_int(tg: &mut TermGenerator, data: i32, prefix: &str, err: &mut i8);
        pub(crate) fn index_long(tg: &mut TermGenerator, data: i64, prefix: &str, err: &mut i8);
        pub(crate) fn index_float(tg: &mut TermGenerator, data: f32, prefix: &str, err: &mut i8);
        pub(crate) fn index_double(tg: &mut TermGenerator, data: f64, prefix: &str, err: &mut i8);

        pub(crate) type Document;
        pub(crate) fn new_document(err: &mut i8) -> UniquePtr<Document>;
        pub(crate) fn add_string(doc: &mut Document, slot: u32, data: &str, err: &mut i8);
        pub(crate) fn add_int(doc: &mut Document, slot: u32, data: i32, err: &mut i8);
        pub(crate) fn add_long(doc: &mut Document, slot: u32, data: i64, err: &mut i8);
        pub(crate) fn add_float(doc: &mut Document, slot: u32, data: f32, err: &mut i8);
        pub(crate) fn add_double(doc: &mut Document, slot: u32, data: f64, err: &mut i8);
        pub(crate) fn set_data(doc: &mut Document, data: &str, err: &mut i8);
        pub(crate) fn add_boolean_term(doc: &mut Document, data: &str, err: &mut i8);

        pub(crate) type Enquire;

        pub(crate) type QueryParser;
        pub(crate) fn new_query_parser(err: &mut i8) -> UniquePtr<QueryParser>;
        pub(crate) fn set_max_wildcard_expansion(qp: &mut QueryParser, limit: i32, err: &mut i8);
        pub(crate) fn set_stemmer_to_qp(qp: &mut QueryParser, stem: &mut Stem, err: &mut i8);
        pub(crate) fn set_database(db: &mut QueryParser, add_db: &mut Database, err: &mut i8);
    }
}

pub struct QueryParser {
    pub cxxp: UniquePtr<ffi::QueryParser>,
}

#[allow(unused_unsafe)]
impl QueryParser {
    pub fn new() -> Result<Self, i8> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_query_parser(&mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(err)
            }
        }
    }

    pub fn set_max_wildcard_expansion(&mut self, limit: i32) -> Result<(), i8> {
        unsafe {
            let mut err = 0;
            ffi::set_max_wildcard_expansion(&mut self.cxxp, limit, &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn set_stemmer(&mut self, stem: &mut Stem) -> Result<(), i8> {
        unsafe {
            let mut err = 0;
            ffi::set_stemmer_to_qp(&mut self.cxxp, &mut stem.cxxp, &mut err);
            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn set_database(&mut self, database: &mut Database) -> Result<(), i8> {
        unsafe {
            let mut err = 0;
            ffi::set_database(&mut self.cxxp, &mut database.cxxp, &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(err)
            }
        }
    }
}

pub struct Enquire {
    pub cxxp: UniquePtr<ffi::Enquire>,
}

pub struct Database {
    pub cxxp: UniquePtr<ffi::Database>,
}

#[allow(unused_unsafe)]
impl Database {
    pub fn new() -> Result<Self, i8> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_database(&mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(err)
            }
        }
    }

    pub fn new_with_path(path: &str, db_type: i8) -> Result<Self, i8> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_database_with_path(path, db_type, &mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(err)
            }
        }
    }

    pub fn add_database(&mut self, database: &mut Database) -> Result<(), i8> {
        unsafe {
            let mut err = 0;
            ffi::add_database(&mut self.cxxp, &mut database.cxxp, &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn reopen(&mut self) -> Result<(), i8> {
        unsafe {
            let mut err = 0;
            ffi::database_reopen(&mut self.cxxp, &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(err)
            }
        }
    }

    pub fn close(&mut self) -> Result<(), i8> {
        unsafe {
            let mut err = 0;
            ffi::database_close(&mut self.cxxp, &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(err)
            }
        }
    }
}

pub struct WritableDatabase {
    cxxp: UniquePtr<ffi::WritableDatabase>,
}

#[allow(unused_unsafe)]
impl WritableDatabase {
    pub fn new(path: &str, action: i8, db_type: i8) -> Result<Self, i8> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_writable_database_with_path(path, action, db_type, &mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(err)
            }
        }
    }

    pub fn delete_document(&mut self, unique_term: &str) -> Result<(), i8> {
        unsafe {
            let mut err = 0;
            ffi::delete_document(&mut self.cxxp, unique_term, &mut err);
            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn replace_document(&mut self, unique_term: &str, doc: &mut Document) -> Result<(), i8> {
        unsafe {
            let mut err = 0;
            ffi::replace_document(&mut self.cxxp, unique_term, &mut doc.cxxp, &mut err);
            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn commit(&mut self) -> Result<(), i8> {
        unsafe {
            let mut err = 0;
            ffi::commit(&mut self.cxxp, &mut err);
            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }
}

pub struct Document {
    cxxp: UniquePtr<ffi::Document>,
}

#[allow(unused_unsafe)]
impl Document {
    pub fn new() -> Result<Self, i8> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_document(&mut err);
            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(err)
            }
        }
    }

    pub fn add_string(&mut self, slot: u32, data: &str) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::add_string(&mut self.cxxp, slot, data, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn add_int(&mut self, slot: u32, data: i32) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::add_int(&mut self.cxxp, slot, data, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn add_long(&mut self, slot: u32, data: i64) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::add_long(&mut self.cxxp, slot, data, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn add_double(&mut self, slot: u32, data: f64) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::add_double(&mut self.cxxp, slot, data, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn set_data(&mut self, data: &str) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::set_data(&mut self.cxxp, data, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn add_boolean_term(&mut self, data: &str) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::add_boolean_term(&mut self.cxxp, data, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }
}

pub struct Stem {
    cxxp: UniquePtr<ffi::Stem>,
}

#[allow(unused_unsafe)]
impl Stem {
    pub fn new(lang: &str) -> Result<Self, i8> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_stem(lang, &mut err);
            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(err)
            }
        }
    }
}

pub struct TermGenerator {
    cxxp: UniquePtr<ffi::TermGenerator>,
}

#[allow(unused_unsafe)]
impl TermGenerator {
    pub fn new() -> Result<Self, i8> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_termgenerator(&mut err);
            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(err)
            }
        }
    }
}

#[allow(unused_unsafe)]
impl TermGenerator {
    pub fn set_stemmer(&mut self, stem: &mut Stem) -> Result<(), i8> {
        unsafe {
            let mut err = 0;
            ffi::set_stemmer(&mut self.cxxp, &mut stem.cxxp, &mut err);
            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn set_document(&mut self, doc: &mut Document) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::set_document(&mut self.cxxp, &mut doc.cxxp, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn index_text_with_prefix(&mut self, data: &str, prefix: &str) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::index_text_with_prefix(&mut self.cxxp, data, prefix, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn index_text(&mut self, data: &str) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::index_text(&mut self.cxxp, data, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn index_int(&mut self, data: i32, prefix: &str) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::index_int(&mut self.cxxp, data, prefix, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn index_long(&mut self, data: i64, prefix: &str) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::index_long(&mut self.cxxp, data, prefix, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn index_float(&mut self, data: f32, prefix: &str) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::index_float(&mut self.cxxp, data, prefix, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }

    pub fn index_double(&mut self, data: f64, prefix: &str) -> Result<(), i8> {
        unsafe {
            let mut err = 0;

            ffi::index_double(&mut self.cxxp, data, prefix, &mut err);

            if err < 0 {
                return Err(err);
            }
        }
        Ok(())
    }
}

pub fn get_xapian_err_type(errcode: i8) -> &'static str {
    match errcode {
        -1 => "DatabaseModifiedError",
        -2 => "DatabaseLockError",
        -3 => "LogicError",
        -4 => "AssertionError",
        -5 => "InvalidArgumentError",
        -6 => "InvalidOperationError",
        -7 => "UnimplementedError",
        -8 => "RuntimeError",
        -9 => "DatabaseError",
        -10 => "DatabaseCorruptError",
        -11 => "DatabaseCreateError",
        -12 => "DatabaseOpeningError",
        -13 => "DatabaseVersionError",
        -14 => "DocNotFoundError",
        -15 => "FeatureUnavailableError",
        -16 => "InternalError",
        -17 => "NetworkError",
        -18 => "NetworkTimeoutError",
        -19 => "QueryParserError",
        -20 => "RangeError",
        -21 => "SerialisationError",
        _ => "Unknown",
    }
}
