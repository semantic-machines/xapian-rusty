#pragma once
#include "rust/cxx.h"
#include <memory>
#include <xapian.h>
#include <string>
#include <string.h>

using namespace Xapian;

std::unique_ptr<Database> new_database(int8_t &err);
std::unique_ptr<Enquire> new_enquire(Database &db, int8_t &err);

//
std::unique_ptr<Database> new_database_with_path(rust::Str path, int8_t db_type, int8_t &err);
void database_reopen (Database &db, int8_t &err);
void add_database(Database &db, Database &add_db, int8_t &err);
void database_close(Database &db, int8_t &err);

//
std::unique_ptr<Stem> new_stem(rust::Str lang, int8_t &err);

//
std::unique_ptr<WritableDatabase> new_writable_database_with_path(rust::Str path, int8_t action, int8_t &err);
void commit (WritableDatabase &db, int8_t &err);
docid replace_document(WritableDatabase &db, rust::Str unique_term, Document &doc,  int8_t &err);
void delete_document(WritableDatabase &db, rust::Str unique_term, int8_t &err);

//
std::unique_ptr<TermGenerator> new_termgenerator(int8_t &err);
void set_stemmer (TermGenerator &tg, Stem &stem, int8_t &err);
void set_document (TermGenerator &tg, Document &doc, int8_t &err);
void index_text_with_prefix (TermGenerator &tg, rust::Str data, rust::Str prefix, int8_t &err);
void index_text (TermGenerator &tg, rust::Str data, int8_t &err);
void index_int (TermGenerator &tg, int32_t data, rust::Str prefix, int8_t &err);
void index_long (TermGenerator &tg, int64_t data, rust::Str prefix, int8_t &err);
void index_float(TermGenerator &tg, float in_data, rust::Str prefix, int8_t &err);
void index_double (TermGenerator &tg, double data, rust::Str prefix, int8_t &err);

//
std::unique_ptr<Document> new_document (int8_t &err);
void add_string (Document &doc, valueno slot, rust::Str data, int8_t &err);
void add_int (Document &doc, valueno slot, int data, int8_t &err);
void add_long(Document &doc, valueno slot, int64_t in_data, int8_t &err);
void add_float(Document &doc, valueno slot, float in_data, int8_t &err);
void add_double(Document &doc, valueno slot, double in_data, int8_t &err);
void set_data (Document &doc, rust::Str data, int8_t &err);
void add_boolean_term(Document &doc, rust::Str data, int8_t &err);
const std::string &get_doc_data (Document &doc);

//
std::unique_ptr<QueryParser> new_query_parser(int8_t &err);
void set_max_wildcard_expansion(QueryParser &qp, int32_t limit, int8_t &err);
void set_stemmer_to_qp(QueryParser &qp, Stem &stem, int8_t &err);
void set_database(QueryParser &qp, Database &db, int8_t &err);
std::unique_ptr<Query> parse_query(QueryParser &qp, rust::Str data, int16_t flags, int8_t &err);
std::unique_ptr<Query> parse_query_with_prefix(QueryParser &qp, rust::Str query, int16_t flags, rust::Str prefix, int8_t &err);

//
std::unique_ptr<Query> new_query(int8_t &err);
std::unique_ptr<Query> new_query_range(int32_t op, valueno slot, double begin, double end, int8_t &err);
std::unique_ptr<Query> new_query_double_with_prefix(rust::Str prefix, double _d, int8_t &err);
std::unique_ptr<Query> add_right_query(Query &this_q, int32_t _op, Query &q, int8_t &err);
bool query_is_empty (Query &q, int8_t &err);
const std::string &get_description (Query &q);

//
std::unique_ptr<MSet> get_mset(Enquire &en, int32_t from, int32_t size, int8_t &err);
void set_query(Enquire &en, Query &query, int8_t &err);
void set_sort_by_key(Enquire &en, MultiValueKeyMaker & sorter, bool reverse, int8_t &err);

//
int get_matches_estimated (MSet &set, int8_t &err);
int mset_size (MSet &set, int8_t &err);
std::unique_ptr<Document> get_doc_by_index (MSet &set, int32_t index, int8_t &err);

//
std::unique_ptr<MultiValueKeyMaker> new_multi_value_key_maker (int8_t &err);
void add_value_to_multi_value_key_maker(MultiValueKeyMaker &this_m, valueno slot, bool asc_desc, int8_t &err);


