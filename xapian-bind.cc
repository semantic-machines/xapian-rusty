#include "xapian-rusty/xapian-bind.h"
#include "xapian-rusty/src/lib.rs.h"
#include <iostream>

#include <xapian.h>
#include <string>
#include <string.h>

using namespace Xapian;

const int BRASS     = 1;
const int CHERT     = 2;
const int IN_MEMORY = 3;

char get_err_code(const char *type)
{
    signed char err = 0;

    if (strcmp(type, (char *)"AssertionError") == 0)
        err = 0;
    if (strcmp(type, (char *)"InvalidArgumentError") == 0)
        err = -1;
    else if (strcmp(type, (char *)"InvalidOperationError") == 0)
        err = -2;
    else if (strcmp(type, (char *)"UnimplementedError") == 0)
        err = -3;
    else if (strcmp(type, (char *)"DatabaseError") == 0)
        err = -4;
    else if (strcmp(type, (char *)"DatabaseCorruptError") == 0)
        err = -5;
    else if (strcmp(type, (char *)"DatabaseCreateError") == 0)
        err = -6;
    else if (strcmp(type, (char *)"DatabaseLockError") == 0)
        err = -7;
    else if (strcmp(type, (char *)"RuntimeError") == 0)
        err = -8;
    else if (strcmp(type, (char *)"DatabaseError") == 0)
        err = -9;
    else if (strcmp(type, (char *)"DatabaseModifiedError") == 0)
        err = -10;
    else if (strcmp(type, (char *)"DatabaseOpeningError") == 0)
        err = -11;
    else if (strcmp(type, (char *)"DatabaseVersionError") == 0)
        err = -12;
    else if (strcmp(type, (char *)"DocNotFoundError") == 0)
        err = -13;
    else if (strcmp(type, (char *)"FeatureUnavailableError") == 0)
        err = -14;
    else if (strcmp(type, (char *)"InternalError") == 0)
        err = -15;
    else if (strcmp(type, (char *)"NetworkError") == 0)
        err = -16;
    else if (strcmp(type, (char *)"NetworkTimeoutError") == 0)
        err = -17;
    else if (strcmp(type, (char *)"QueryParserError") == 0)
        err = -20;
    else if (strcmp(type, (char *)"SerialisationError") == 0)
        err = -21;
    else if (strcmp(type, (char *)"RangeError") == 0)
        err = -22;
    else if (strcmp(type, (char *)"WildcardError") == 0)
        err = -23;
    else if (strcmp(type, (char *)"DatabaseNotFoundError") == 0)
        err = -24;
    else if (strcmp(type, (char *)"DatabaseClosedError") == 0)
        err = -25;

    return err;
}

std::unique_ptr<Database> new_database(int8_t &err)
{
    try
    {
        err = 0;
        return std::make_unique<Xapian::Database>();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

std::unique_ptr<Database> new_database_with_path(rust::Str path, int8_t db_type, int8_t &err)
{
    try
    {
        err = 0;

        if (db_type == CHERT)
            return std::make_unique<Database>(Chert::open(std::string(path)));
        else if (db_type == IN_MEMORY)
            return std::make_unique<Database>(InMemory::open());
        else
            return std::make_unique<Database>(std::string(path));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

void add_database(Database &db, Database &add_db, int8_t &err)
{
    try
    {
        db.add_database(add_db);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void database_close(Database &db, int8_t &err)
{
    try
    {
        db.close();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void database_reopen(Database &db, int8_t &err)
{
    try
    {
        db.reopen();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

std::unique_ptr<Enquire> new_enquire(Database &db, int8_t &err)
{
    try
    {
        err = 0;
        return std::make_unique<Xapian::Enquire>(db);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}


//////

std::unique_ptr<Stem> new_stem(rust::Str lang, int8_t &err)
{
    try
    {
        err = 0;
        return std::make_unique<Stem>(std::string(lang));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

///////////////////////////////////////////////////////////////
std::unique_ptr<WritableDatabase> new_writable_database_with_path(rust::Str path, int8_t action, int8_t db_type, int8_t &err)
{
    try
    {
        err = 0;

        if (db_type == CHERT) {
            return std::make_unique<WritableDatabase>(Chert::open(std::string(path), action));
        }
        else if (db_type == IN_MEMORY)
            return std::make_unique<WritableDatabase>(InMemory::open());
        else {
            return std::make_unique<WritableDatabase>(std::string(path), action);
        }
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        if (err == 0) {
            err = -4;
        }
        return NULL;
    }
}

void commit(WritableDatabase &db, int8_t &err)
{
    try
    {
        db.commit();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

int32_t get_doccount (WritableDatabase &db, int8_t &err) {
    try
    {
        return db.get_doccount();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return 0;
    }
}

docid replace_document(WritableDatabase &db, rust::Str unique_term, Document &doc, int8_t &err)
{
    try
    {
        return db.replace_document(std::string(unique_term), doc);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return -1;
    }
}

void delete_document(WritableDatabase &db, rust::Str unique_term, int8_t &err)
{
    try
    {
        db.delete_document(std::string(unique_term));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}


////////////////////////////////////////////////////////////////

std::unique_ptr<TermGenerator> new_termgenerator(int8_t &err)
{
    try
    {
        err = 0;
        return std::make_unique<TermGenerator>();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

void set_stemmer(TermGenerator &tg, Stem &stem, int8_t &err)
{
    try
    {
        tg.set_stemmer(stem);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void set_document(TermGenerator &tg, Document &doc, int8_t &err)
{
    try
    {
        err = 0;
        tg.set_document(doc);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void index_text(TermGenerator &tg, rust::Str data, int8_t &err)
{
    try
    {
        err = 0;
        tg.index_text(std::string(data));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void index_text_with_prefix(TermGenerator &tg, rust::Str data, rust::Str prefix, int8_t &err)
{
    try
    {
        err = 0;
        tg.index_text(std::string(data), 1, std::string(prefix));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void index_int(TermGenerator &tg, int32_t in_data, rust::Str prefix, int8_t &err)
{
    try
    {
        err = 0;
        std::string data = sortable_serialise(in_data);
        tg.index_text(data, 1, std::string(prefix));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void index_long(TermGenerator &tg, int64_t in_data, rust::Str prefix, int8_t &err)
{
    try
    {
        err = 0;
        std::string data = sortable_serialise(in_data);
        tg.index_text(data, 1, std::string(prefix));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void index_float(TermGenerator &tg, float in_data, rust::Str prefix, int8_t &err)
{
    try
    {
        err = 0;
        std::string data = sortable_serialise(in_data);
        tg.index_text(data, 1, std::string(prefix));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void index_double(TermGenerator &tg, double in_data, rust::Str prefix, int8_t &err)
{
    try
    {
        err = 0;
        std::string data = sortable_serialise(in_data);
        tg.index_text(data, 1, std::string(prefix));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

////////////////////////////////////////////////////////////////

std::unique_ptr<Document> new_document(int8_t &err)
{
    try
    {
        err = 0;
        return std::make_unique<Document>();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

void add_string(Document &doc, valueno slot, rust::Str data, int8_t &err)
{
    try
    {
        err = 0;
        doc.add_value(slot, std::string(data));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void add_int(Document &doc, valueno slot, int in_data, int8_t &err)
{
    try
    {
        err = 0;
        std::string data = sortable_serialise(in_data);
        doc.add_value(slot, data);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void add_long(Document &doc, valueno slot, int64_t in_data, int8_t &err)
{
    try
    {
        err = 0;
        std::string data = sortable_serialise(in_data);
        doc.add_value(slot, data);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void add_float(Document &doc, valueno slot, float in_data, int8_t &err)
{
    try
    {
        err = 0;
        std::string data = sortable_serialise(in_data);
        doc.add_value(slot, data);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void add_double(Document &doc, valueno slot, double in_data, int8_t &err)
{
    try
    {
        err = 0;
        std::string data = sortable_serialise(in_data);
        doc.add_value(slot, data);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void set_data(Document &doc, rust::Str data, int8_t &err)
{
    try
    {
        err = 0;
        doc.set_data(std::string(data));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void add_boolean_term(Document &doc, rust::Str data, int8_t &err)
{
    try
    {
        err = 0;
        doc.add_boolean_term(std::string(data));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

rust::String get_doc_data (Document &doc) {
    try
    {
        return doc.get_data();
    }
    catch (...)
    {
        return rust::String("");
    }

}

//////

std::unique_ptr<QueryParser> new_query_parser(int8_t &err)
{
    try
    {
        err = 0;
        return std::make_unique<Xapian::QueryParser>();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

void set_max_wildcard_expansion(QueryParser &qp, int32_t limit, int8_t &err) {
    try
    {
        err = 0;
        qp.set_max_wildcard_expansion (limit);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return;
    }
}

void set_stemmer_to_qp(QueryParser &qp, Stem &stem, int8_t &err) {
    try
    {
        err = 0;
        qp.set_stemmer(stem);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void set_database(QueryParser &qp, Database &db, int8_t &err)
{
    try
    {
        err = 0;
        qp.set_database(db);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

std::unique_ptr<Query> parse_query(QueryParser &qp, rust::Str data, int16_t flags, int8_t &err) {
    try
    {
        err = 0;
        return std::make_unique<Xapian::Query>(qp.parse_query(std::string(data), flags));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

std::unique_ptr<Query> parse_query_with_prefix(QueryParser &qp, rust::Str query, int16_t flags, rust::Str prefix, int8_t &err) {
    try
    {
        err = 0;
        return std::make_unique<Xapian::Query>(qp.parse_query(std::string(query), flags, std::string(prefix)));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

////////

std::unique_ptr<Query> new_query(int8_t &err) {
    try
    {
        err = 0;
        return std::make_unique<Xapian::Query>();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

std::unique_ptr<Query> new_query_range(int32_t _op, valueno slot, double _begin, double _end, int8_t &err) {
    try
    {
        err = 0;

        std::string s_begin = Xapian::sortable_serialise(_begin);
        std::string s_end = Xapian::sortable_serialise(_end);
        Xapian::Query _query ((Xapian::Query::op)_op, slot, s_begin, s_end);

        return std::make_unique<Xapian::Query>(_query);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

std::unique_ptr<Query> add_right_query(Query &this_q, int32_t _op, Query &q, int8_t &err) {
    try
    {
        err = 0;
        return std::make_unique<Xapian::Query>((Xapian::Query::op)_op, this_q, q);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

std::unique_ptr<Query> new_query_double_with_prefix(rust::Str prefix, double _d, int8_t &err) {
    try
    {
        err = 0;

        std::string s = std::string(prefix) + Xapian::sortable_serialise(_d);

        Xapian::Query _query (s);
        return std::make_unique<Xapian::Query>(_query);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

bool query_is_empty (Query &q, int8_t &err) {
    try
    {
        err = 0;
        return q.empty();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return true;
    }
}

std::string g_str_1;
const std::string &get_description (Query &q) {
    try
    {
        //err = 0;
        g_str_1 = q.get_description();
        return g_str_1;
    }
    catch (Error ex)
    {
        //err = get_err_code(ex.get_type());
        return NULL;
    }
}

////

std::unique_ptr<MSet> get_mset(Enquire &en, int32_t from, int32_t size, int8_t &err) {
    try
    {
        err = 0;
        return std::make_unique<Xapian::MSet>(en.get_mset(from, size));
    }
    catch (Xapian::DatabaseModifiedError &e) {
        err = -10;
        return NULL;
    }
    catch (Error ex){
        err = get_err_code(ex.get_type());
        return NULL;
    }
    catch (...) {
        err = -15;
        return NULL;
    }
}

void set_query(Enquire &en, Query &query, int8_t &err) {
    try
    {
        err = 0;
        en.set_query(query);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

void set_sort_by_key(Enquire &en, MultiValueKeyMaker &sorter, bool reverse, int8_t &err) {
    try
    {
        err = 0;
        en.set_sort_by_key(&sorter, reverse);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}

/////

int get_matches_estimated (MSet &set, int8_t &err) {
    try
    {
        err = 0;
        return set.get_matches_estimated();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return -1;
    }
}

int mset_size (MSet &set, int8_t &err) {
    try
    {
        err = 0;
        return set.size();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return -1;
    }
}

std::unique_ptr<Document> get_doc_by_index (MSet &set, int32_t index, int8_t &err) {
    try
    {
        err = 0;
        return std::make_unique<Xapian::Document>(set.get_doc_by_index(index));
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

/////

std::unique_ptr<MultiValueKeyMaker> new_multi_value_key_maker (int8_t &err) {
    try
    {
        err = 0;
        return std::make_unique<Xapian::MultiValueKeyMaker>();
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
        return NULL;
    }
}

void add_value_to_multi_value_key_maker(MultiValueKeyMaker &this_m, valueno slot, bool asc_desc, int8_t &err) {
    try
    {
        err = 0;
        this_m.add_value(slot, asc_desc);
    }
    catch (Error ex)
    {
        err = get_err_code(ex.get_type());
    }
}
