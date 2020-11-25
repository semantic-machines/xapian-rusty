use std::error::Error as StdError;
use std::fmt::{self, Display};
use std::io;

use cxx::UniquePtr;

pub const BRASS: i8 = 1;
pub const CHERT: i8 = 2;
pub const IN_MEMORY: i8 = 3;
pub const UNKNOWN: i8 = 0;

pub const DB_CREATE_OR_OPEN: i8 = 1;
/** Create a new database; fail if db exists. */
pub const DB_CREATE: i8 = 2;
/** Overwrite existing db; create if none exists. */
pub const DB_CREATE_OR_OVERWRITE: i8 = 3;
/** Open for read/write; fail if no db exists. */
pub const DB_OPEN: i8 = 4;

/// Enum of possible query operations
/// #[repr(i32)]
pub enum XapianOp {
    /// Return iff both subqueries are satisfied
    OpAnd,

    /// Return if either subquery is satisfied
    OpOr,

    /// Return if left but not right satisfied
    OpAndNot,

    /// Return if one query satisfied, but not both
    OpXor,

    /// Return iff left satisfied, but use weights from both
    OpAndMaybe,

    /// As AND, but use only weights from left subquery
    OpFilter,

    /** Find occurrences of a list of terms with all the terms
     *  occurring within a specified window of positions.
     *
     *  Each occurrence of a term must be at a different position,
     *  but the order they appear in is irrelevant.
     *
     *  The window parameter should be specified for this operation,
     *  but will default to the number of terms in the list.
     */
    OpNear,

    /** Find occurrences of a list of terms with all the terms
     *  occurring within a specified window of positions, and all
     *  the terms appearing in the order specified.
     *
     *  Each occurrence of a term must be at a different position.
     *
     *  The window parameter should be specified for this operation,
     *  but will default to the number of terms in the list.
     */
    OpPhrase,

    /** Filter by a range test on a document value. */
    OpValueRange,

    /** Scale the weight of a subquery by the specified factor.
     *
     *  A factor of 0 means this subquery will contribute no weight to
     *  the query - it will act as a purely boolean subquery.
     *
     *  If the factor is negative, Xapian::InvalidArgumentError will
     *  be thrown.
     */
    OpScaleWeight,

    /** Pick the best N subqueries and combine with OP_OR.
     *
     *  If you want to implement a feature which finds documents
     *  similar to a piece of text, an obvious approach is to build an
     *  "OR" query from all the terms in the text, and run this query
     *  against a database containing the documents.  However such a
     *  query can contain a lots of terms and be quite slow to perform,
     *  yet many of these terms don't contribute usefully to the
     *  results.
     *
     *  The OP_ELITE_SET operator can be used instead of OP_OR in this
     *  situation.  OP_ELITE_SET selects the most important ''N'' terms
     *  and then acts as an OP_OR query with just these, ignoring any
     *  other terms.  This will usually return results just as good as
     *  the full OP_OR query, but much faster.
     *
     *  In general, the OP_ELITE_SET operator can be used when you have
     *  a large OR query, but it doesn't matter if the search
     *  completely ignores some of the less important terms in the
     *  query.
     *
     *  The subqueries don't have to be terms, but if they aren't then
     *  OP_ELITE_SET will look at the estimated frequencies of the
     *  subqueries and so could pick a subset which don't actually
     *  match any documents even if the full OR would match some.
     *
     *  You can specify a parameter to the query constructor which
     *  control the number of terms which OP_ELITE_SET will pick.  If
     *  not specified, this defaults to 10 (or
     *  <code>ceil(sqrt(number_of_subqueries))</code> if there are more
     *  than 100 subqueries, but this rather arbitrary special case
     *  will be dropped in 1.3.0).  For example, this will pick the
     *  best 7 terms:
     *
     *  <pre>
     *  Xapian::Query query(Xapian::Query::OP_ELITE_SET, subqs.begin(), subqs.end(), 7);
     *  </pre>
     *
     * If the number of subqueries is less than this threshold,
     * OP_ELITE_SET behaves identically to OP_OR.
     */
    OpEliteSet,

    /** Filter by a greater-than-or-equal test on a document value. */
    OpValueGe,

    /** Filter by a less-than-or-equal test on a document value. */
    OpValueLe,

    /** Treat a set of queries as synonyms.
     *
     *  This returns all results which match at least one of the
     *  queries, but weighting as if all the sub-queries are instances
     *  of the same term: so multiple matching terms for a document
     *  increase the wdf value used, and the term frequency is based on
     *  the number of documents which would match an OR of all the
     *  subqueries.
     *
     *  The term frequency used will usually be an approximation,
     *  because calculating the precise combined term frequency would
     *  be overly expensive.
     *
     *  Identical to OP_OR, except for the weightings returned.
     */
    OpSynonym,
}

/// Enum of feature flag
#[repr(i16)]
pub enum FeatureFlag {
    /// Support AND, OR, etc and bracketed subexpressions.
    FlagBoolean = 1,
    /// Support quoted phrases.
    FlagPhrase = 2,
    /// Support + and -.
    FlagLovehate = 4,
    /// Support AND, OR, etc even if they aren't in ALLCAPS.
    FlagBooleanAnyCase = 8,
    /** Support right truncation (e.g. Xap*).
     *
     *  Currently you can't use wildcards with boolean filter prefixes,
     *  or in a phrase (either an explicitly quoted one, or one implicitly
     *  generated by hyphens or other punctuation).
     *
     *  NB: You need to tell the QueryParser object which database to
     *  expand wildcards from by calling set_database.
     */
    FlagWildcard = 16,
    /** Allow queries such as 'NOT apples'.
     *
     *  These require the use of a list of all documents in the database
     *  which is potentially expensive, so this feature isn't enabled by
     *  default.
     */
    FlagPureNot = 32,
    /** Enable partial matching.
     *
     *  Partial matching causes the parser to treat the query as a
     *  "partially entered" search.  This will automatically treat the
     *  final word as a wildcarded match, unless it is followed by
     *  whitespace, to produce more stable results from interactive
     *  searches.
     *
     *  Currently FLAG_PARTIAL doesn't do anything if the final word
     *  in the query has a boolean filter prefix, or if it is in a phrase
     *  (either an explicitly quoted one, or one implicitly generated by
     *  hyphens or other punctuation).  It also doesn't do anything if
     *  if the final word is part of a value range.
     *
     *  NB: You need to tell the QueryParser object which database to
     *  expand wildcards from by calling set_database.
     */
    FlagPartial = 64,

    /** Enable spelling correction.
     *
     *  For each word in the query which doesn't exist as a term in the
     *  database, Database::get_spelling_suggestion() will be called and if
     *  a suggestion is returned, a corrected version of the query string
     *  will be built up which can be read using
     *  QueryParser::get_corrected_query_string().  The query returned is
     *  based on the uncorrected query string however - if you want a
     *  parsed query based on the corrected query string, you must call
     *  QueryParser::parse_query() again.
     *
     *  NB: You must also call set_database() for this to work.
     */
    FlagSpellingCorrection = 128,

    /** Enable synonym operator '~'.
     *
     *  NB: You must also call set_database() for this to work.
     */
    FlagSynonym = 256,

    /** Enable automatic use of synonyms for single terms.
     *
     *  NB: You must also call set_database() for this to work.
     */
    FlagAutoSynonyms = 512,

    /** Enable automatic use of synonyms for single terms and groups of
     *  terms.
     *
     *  NB: You must also call set_database() for this to work.
     */
    FlagAutoMultiwordSynonyms = 1024 | FeatureFlag::FlagAutoSynonyms as i16,

    /** The default flags.
     *
     *  Used if you don't explicitly pass any to @a parse_query().
     *  The default flags are FLAG_PHRASE|FLAG_BOOLEAN|FLAG_LOVEHATE.
     *
     *  Added in Xapian 1.0.11.
     */
    FlagDefault = FeatureFlag::FlagPhrase as i16 | FeatureFlag::FlagBoolean as i16 | FeatureFlag::FlagLovehate as i16,
}

//use cxx::CxxString;

#[cxx::bridge]
pub(crate) mod ffi {
    #[namespace = "Xapian"]
    extern "C++" {
        pub(crate) type Database;
        pub(crate) type Stem;
        pub(crate) type WritableDatabase;
        pub(crate) type TermGenerator;
        pub(crate) type Document;
        pub(crate) type MSet;
        pub(crate) type Enquire;
        pub(crate) type QueryParser;
        pub(crate) type Query;
        pub(crate) type MultiValueKeyMaker;
    }

    unsafe extern "C++" {
        include!("xapian-rusty/xapian-bind.h");

        pub(crate) fn new_database(err: &mut i8) -> UniquePtr<Database>;
        pub(crate) fn new_database_with_path(path: &str, db_type: i8, err: &mut i8) -> UniquePtr<Database>;
        pub(crate) fn database_reopen(db: Pin<&mut Database>, err: &mut i8);
        pub(crate) fn database_close(db: Pin<&mut Database>, err: &mut i8);
        pub(crate) fn new_enquire(db: Pin<&mut Database>, err: &mut i8) -> UniquePtr<Enquire>;
        pub(crate) fn add_database(db: Pin<&mut Database>, add_db: Pin<&mut Database>, err: &mut i8);

        pub(crate) fn new_stem(lang: &str, err: &mut i8) -> UniquePtr<Stem>;

        pub(crate) fn new_writable_database_with_path(path: &str, action: i8, db_type: i8, err: &mut i8) -> UniquePtr<WritableDatabase>;
        pub(crate) fn commit(db: Pin<&mut WritableDatabase>, err: &mut i8);
        pub(crate) fn replace_document(db: Pin<&mut WritableDatabase>, unique_term: &str, doc: Pin<&mut Document>, err: &mut i8) -> u32;
        pub(crate) fn delete_document(db: Pin<&mut WritableDatabase>, unique_term: &str, err: &mut i8);
        pub(crate) fn get_doccount(db: Pin<&mut WritableDatabase>, err: &mut i8) -> i32;

        pub(crate) fn new_termgenerator(err: &mut i8) -> UniquePtr<TermGenerator>;
        pub(crate) fn set_stemmer(tg: Pin<&mut TermGenerator>, stem: Pin<&mut Stem>, err: &mut i8);
        pub(crate) fn set_document(tg: Pin<&mut TermGenerator>, doc: Pin<&mut Document>, err: &mut i8);
        pub(crate) fn index_text_with_prefix(tg: Pin<&mut TermGenerator>, data: &str, prefix: &str, err: &mut i8);
        pub(crate) fn index_text(tg: Pin<&mut TermGenerator>, data: &str, err: &mut i8);
        pub(crate) fn index_int(tg: Pin<&mut TermGenerator>, data: i32, prefix: &str, err: &mut i8);
        pub(crate) fn index_long(tg: Pin<&mut TermGenerator>, data: i64, prefix: &str, err: &mut i8);
        pub(crate) fn index_float(tg: Pin<&mut TermGenerator>, data: f32, prefix: &str, err: &mut i8);
        pub(crate) fn index_double(tg: Pin<&mut TermGenerator>, data: f64, prefix: &str, err: &mut i8);

        pub(crate) fn new_document(err: &mut i8) -> UniquePtr<Document>;
        pub(crate) fn add_string(doc: Pin<&mut Document>, slot: u32, data: &str, err: &mut i8);
        pub(crate) fn add_int(doc: Pin<&mut Document>, slot: u32, data: i32, err: &mut i8);
        pub(crate) fn add_long(doc: Pin<&mut Document>, slot: u32, data: i64, err: &mut i8);
        pub(crate) fn add_float(doc: Pin<&mut Document>, slot: u32, data: f32, err: &mut i8);
        pub(crate) fn add_double(doc: Pin<&mut Document>, slot: u32, data: f64, err: &mut i8);
        pub(crate) fn set_data(doc: Pin<&mut Document>, data: &str, err: &mut i8);
        pub(crate) fn get_doc_data(doc: Pin<&mut Document>) -> &CxxString;
        pub(crate) fn add_boolean_term(doc: Pin<&mut Document>, data: &str, err: &mut i8);

        pub(crate) fn get_matches_estimated(set: Pin<&mut MSet>, err: &mut i8) -> i32;
        pub(crate) fn mset_size(set: Pin<&mut MSet>, err: &mut i8) -> i32;
        pub(crate) fn get_doc_by_index(set: Pin<&mut MSet>, index: i32, err: &mut i8) -> UniquePtr<Document>;

        pub(crate) fn get_mset(en: Pin<&mut Enquire>, from: i32, size: i32, err: &mut i8) -> UniquePtr<MSet>;
        pub(crate) fn set_query(en: Pin<&mut Enquire>, query: Pin<&mut Query>, err: &mut i8);
        pub(crate) fn set_sort_by_key(en: Pin<&mut Enquire>, sorter: Pin<&mut MultiValueKeyMaker>, reverse: bool, err: &mut i8);

        pub(crate) fn new_query_parser(err: &mut i8) -> UniquePtr<QueryParser>;
        pub(crate) fn set_max_wildcard_expansion(qp: Pin<&mut QueryParser>, limit: i32, err: &mut i8);
        pub(crate) fn set_stemmer_to_qp(qp: Pin<&mut QueryParser>, stem: Pin<&mut Stem>, err: &mut i8);
        pub(crate) fn set_database(qp: Pin<&mut QueryParser>, add_db: Pin<&mut Database>, err: &mut i8);
        pub(crate) fn parse_query(qp: Pin<&mut QueryParser>, query_string: &str, flags: i16, err: &mut i8) -> UniquePtr<Query>;
        pub(crate) fn parse_query_with_prefix(qp: Pin<&mut QueryParser>, query_string: &str, flags: i16, prefix: &str, err: &mut i8) -> UniquePtr<Query>;

        pub(crate) fn new_query(err: &mut i8) -> UniquePtr<Query>;
        pub(crate) fn new_query_range(op: i32, slot: u32, begin: f64, end: f64, err: &mut i8) -> UniquePtr<Query>;
        pub(crate) fn add_right_query(this_q: Pin<&mut Query>, op: i32, q: Pin<&mut Query>, err: &mut i8) -> UniquePtr<Query>;
        pub(crate) fn new_query_double_with_prefix(prefix: &str, d: f64, err: &mut i8) -> UniquePtr<Query>;
        pub(crate) fn query_is_empty(this_q: Pin<&mut Query>, err: &mut i8) -> bool;
        pub(crate) fn get_description(this_q: Pin<&mut Query>) -> &CxxString;

        pub(crate) fn new_multi_value_key_maker(err: &mut i8) -> UniquePtr<MultiValueKeyMaker>;
        pub(crate) fn add_value_to_multi_value_key_maker(this_m: Pin<&mut MultiValueKeyMaker>, slot: u32, asc_desc: bool, err: &mut i8);
    }
}

#[warn(unused_unsafe)]

pub struct MultiValueKeyMaker {
    pub cxxp: UniquePtr<ffi::MultiValueKeyMaker>,
}

impl MultiValueKeyMaker {
    pub fn new() -> Result<Self, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let obj = ffi::new_multi_value_key_maker(&mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn add_value(&mut self, slot: u32, asc_desc: bool) -> Result<(), XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            ffi::add_value_to_multi_value_key_maker(self.cxxp.pin_mut(), slot, asc_desc, &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }
}

pub struct Query {
    pub cxxp: UniquePtr<ffi::Query>,
}

impl Query {
    pub fn new() -> Result<Self, XError> {
        Ok(Self {
            cxxp: UniquePtr::null(),
        })
    }

    pub fn new_range(op: XapianOp, slot: u32, begin: f64, end: f64) -> Result<Self, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let obj = ffi::new_query_range(op as i32, slot, begin, end, &mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn add_right(&mut self, op: XapianOp, q: &mut Query) -> Result<Self, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let obj = ffi::add_right_query(self.cxxp.pin_mut(), op as i32, q.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn new_double_with_prefix(prefix: &str, d: f64) -> Result<Self, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let obj = ffi::new_query_double_with_prefix(prefix, d, &mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn is_empty(&mut self) -> bool {
        self.cxxp.is_null()
    }

    pub fn is_empty_content_query(&mut self) -> bool {
        if !self.cxxp.is_null() {
            #[allow(unused_unsafe)]
            unsafe {
                let mut err = 0;
                let res = ffi::query_is_empty(self.cxxp.pin_mut(), &mut err);
                if err == 0 {
                    return res;
                } else {
                    return true;
                }
            }
        }
        true
    }

    pub fn get_description(&mut self) -> String {
        if !self.cxxp.is_null() {
            #[allow(unused_unsafe)]
            unsafe {
                //let mut err = 0;
                let res = ffi::get_description(self.cxxp.pin_mut());
                //if err == 0 {
                return res.to_string();
                //} else {
                //    None
                //}
            }
        }
        String::default()
    }
}

pub struct QueryParser {
    pub cxxp: UniquePtr<ffi::QueryParser>,
}

#[allow(unused_unsafe)]
impl QueryParser {
    pub fn new() -> Result<Self, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_query_parser(&mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn set_max_wildcard_expansion(&mut self, limit: i32) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::set_max_wildcard_expansion(self.cxxp.pin_mut(), limit, &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn set_stemmer(&mut self, stem: &mut Stem) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::set_stemmer_to_qp(self.cxxp.pin_mut(), stem.cxxp.pin_mut(), &mut err);
            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn set_database(&mut self, database: &mut Database) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::set_database(self.cxxp.pin_mut(), database.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn parse_query(&mut self, query: &str, flags: i16) -> Result<Query, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::parse_query(self.cxxp.pin_mut(), query, flags, &mut err);
            if err == 0 {
                Ok(Query {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn parse_query_with_prefix(&mut self, query: &str, flags: i16, prefix: &str) -> Result<Query, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::parse_query_with_prefix(self.cxxp.pin_mut(), query, flags, prefix, &mut err);
            if err == 0 {
                Ok(Query {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }
}

pub struct MSetIterator<'a> {
    pub mset: &'a mut MSet,
    pub index: i32,
}

impl<'a> MSetIterator<'a> {
    pub fn is_next(&mut self) -> Result<bool, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let res = ffi::mset_size(self.mset.cxxp.pin_mut(), &mut err) > self.index;

            if err == 0 {
                Ok(res)
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn next(&mut self) -> Result<(), XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            if ffi::mset_size(self.mset.cxxp.pin_mut(), &mut err) > self.index {
                self.index += 1;
            }

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn get_document_data(&mut self) -> Result<String, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let mut doc = ffi::get_doc_by_index(self.mset.cxxp.pin_mut(), self.index, &mut err);

            if err == 0 {
                Ok(ffi::get_doc_data(doc.pin_mut()).to_string())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }
}

pub struct MSet {
    pub cxxp: UniquePtr<ffi::MSet>,
}

impl MSet {
    pub fn iterator(&mut self) -> Result<MSetIterator, i8> {
        Ok(MSetIterator {
            mset: self,
            index: 0,
        })
    }

    pub fn get_matches_estimated(&mut self) -> Result<i32, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let res = ffi::get_matches_estimated(self.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(res)
            } else {
                Err(XError::Xapian(err))
            }
        }
    }
}

pub struct Enquire {
    pub cxxp: UniquePtr<ffi::Enquire>,
    sorter: Option<MultiValueKeyMaker>,
}

impl Enquire {
    pub fn get_mset(&mut self, from: i32, size: i32) -> Result<MSet, XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            let obj = ffi::get_mset(self.cxxp.pin_mut(), from, size, &mut err);

            if err == 0 {
                Ok(MSet {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn set_query(&mut self, query: &mut Query) -> Result<(), XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            ffi::set_query(self.cxxp.pin_mut(), query.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn set_sort_by_key(&mut self, mut sorter: MultiValueKeyMaker, reverse: bool) -> Result<(), XError> {
        #[allow(unused_unsafe)]
        unsafe {
            let mut err = 0;
            ffi::set_sort_by_key(self.cxxp.pin_mut(), sorter.cxxp.pin_mut(), reverse, &mut err);
            self.sorter = Some(sorter);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }
}

pub struct Database {
    pub cxxp: UniquePtr<ffi::Database>,
}

#[allow(unused_unsafe)]
impl Database {
    pub fn new() -> Result<Self, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_database(&mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn new_with_path(path: &str, db_type: i8) -> Result<Self, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_database_with_path(path, db_type, &mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn new_enquire(&mut self) -> Result<Enquire, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_enquire(self.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(Enquire {
                    cxxp: obj,
                    sorter: None,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn add_database(&mut self, database: &mut Database) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::add_database(self.cxxp.pin_mut(), database.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn reopen(&mut self) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::database_reopen(self.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn close(&mut self) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::database_close(self.cxxp.pin_mut(), &mut err);

            if err == 0 {
                Ok(())
            } else {
                Err(XError::Xapian(err))
            }
        }
    }
}

pub struct WritableDatabase {
    cxxp: UniquePtr<ffi::WritableDatabase>,
}

#[allow(unused_unsafe)]
impl WritableDatabase {
    pub fn new(path: &str, action: i8, db_type: i8) -> Result<Self, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_writable_database_with_path(path, action, db_type, &mut err);

            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn delete_document(&mut self, unique_term: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::delete_document(self.cxxp.pin_mut(), unique_term, &mut err);
            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn replace_document(&mut self, unique_term: &str, doc: &mut Document) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::replace_document(self.cxxp.pin_mut(), unique_term, doc.cxxp.pin_mut(), &mut err);
            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn commit(&mut self) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::commit(self.cxxp.pin_mut(), &mut err);
            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn get_doccount(&mut self) -> Result<i32, XError> {
        unsafe {
            let mut err = 0;
            let res = ffi::get_doccount(self.cxxp.pin_mut(), &mut err);
            if err < 0 {
                return Err(XError::Xapian(err));
            } else {
                Ok(res)
            }
        }
    }
}

pub struct Document {
    cxxp: UniquePtr<ffi::Document>,
}

#[allow(unused_unsafe)]
impl Document {
    pub fn new() -> Result<Self, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_document(&mut err);
            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }

    pub fn add_string(&mut self, slot: u32, data: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::add_string(self.cxxp.pin_mut(), slot, data, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn add_int(&mut self, slot: u32, data: i32) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::add_int(self.cxxp.pin_mut(), slot, data, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn add_long(&mut self, slot: u32, data: i64) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::add_long(self.cxxp.pin_mut(), slot, data, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn add_double(&mut self, slot: u32, data: f64) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::add_double(self.cxxp.pin_mut(), slot, data, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn set_data(&mut self, data: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::set_data(self.cxxp.pin_mut(), data, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn add_boolean_term(&mut self, data: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::add_boolean_term(self.cxxp.pin_mut(), data, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
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
    pub fn new(lang: &str) -> Result<Self, XError> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_stem(lang, &mut err);
            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }
}

pub struct TermGenerator {
    cxxp: UniquePtr<ffi::TermGenerator>,
}

#[allow(unused_unsafe)]
impl TermGenerator {
    pub fn new() -> Result<Self> {
        unsafe {
            let mut err = 0;
            let obj = ffi::new_termgenerator(&mut err);
            if err == 0 {
                Ok(Self {
                    cxxp: obj,
                })
            } else {
                Err(XError::Xapian(err))
            }
        }
    }
}

#[allow(unused_unsafe)]
impl TermGenerator {
    pub fn set_stemmer(&mut self, stem: &mut Stem) -> Result<(), XError> {
        unsafe {
            let mut err = 0;
            ffi::set_stemmer(self.cxxp.pin_mut(), stem.cxxp.pin_mut(), &mut err);
            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn set_document(&mut self, doc: &mut Document) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::set_document(self.cxxp.pin_mut(), doc.cxxp.pin_mut(), &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn index_text_with_prefix(&mut self, data: &str, prefix: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::index_text_with_prefix(self.cxxp.pin_mut(), data, prefix, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn index_text(&mut self, data: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::index_text(self.cxxp.pin_mut(), data, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn index_int(&mut self, data: i32, prefix: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::index_int(self.cxxp.pin_mut(), data, prefix, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn index_long(&mut self, data: i64, prefix: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::index_long(self.cxxp.pin_mut(), data, prefix, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn index_float(&mut self, data: f32, prefix: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::index_float(self.cxxp.pin_mut(), data, prefix, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }

    pub fn index_double(&mut self, data: f64, prefix: &str) -> Result<(), XError> {
        unsafe {
            let mut err = 0;

            ffi::index_double(self.cxxp.pin_mut(), data, prefix, &mut err);

            if err < 0 {
                return Err(XError::Xapian(err));
            }
        }
        Ok(())
    }
}

pub fn get_xapian_err_type(errcode: i8) -> &'static str {
    match errcode {
        0 => "AssertionError",
        -1 => "InvalidArgumentError",
        -2 => "InvalidOperationError",
        -3 => "UnimplementedError",
        -4 => "DatabaseError",
        -5 => "DatabaseCorruptError",
        -6 => "DatabaseCreateError",
        -7 => "DatabaseLockError",
        -10 => "DatabaseModifiedError",
        -11 => "DatabaseOpeningError",
        -12 => "DatabaseVersionError",
        -13 => "DocNotFoundError",
        -14 => "FeatureUnavailableError",
        -15 => "InternalError",
        -16 => "NetworkError",
        -17 => "NetworkTimeoutError",
        -20 => "QueryParserError",
        -21 => "SerialisationError",
        -22 => "RangeError",
        -23 => "WildcardError",
        -24 => "DatabaseNotFoundError",
        -25 => "DatabaseClosedError",
        _ => "Unknown",
    }
}

pub type Result<T, E = XError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum XError {
    Xapian(i8),
    Io(io::Error),
}

impl Display for XError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            XError::Xapian(err) => write!(f, "xapian err={}", err),
            XError::Io(err) => err.fmt(f),
        }
    }
}

impl StdError for XError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            XError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for XError {
    fn from(err: io::Error) -> Self {
        XError::Io(err)
    }
}

impl From<i8> for XError {
    fn from(err: i8) -> Self {
        XError::Xapian(err)
    }
}

impl From<XError> for i8 {
    fn from(err: XError) -> i8 {
        err.into()
    }
}
