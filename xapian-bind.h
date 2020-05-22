#pragma once
#include "rust/cxx.h"
#include <memory>
#include <xapian.h>
#include <string>
#include <string.h>

using namespace Xapian;

std::unique_ptr<Database> new_database(int8_t &err);

//
std::unique_ptr<Database> new_database_with_path(rust::Str path, int8_t db_type, int8_t &err);
void database_reopen (Database &db, int8_t &err);

//
std::unique_ptr<Stem> new_stem(rust::Str lang, int8_t &err);

//
std::unique_ptr<WritableDatabase> new_writable_database_with_path(rust::Str path, int8_t action, int8_t db_type, int8_t &err);
void commit (WritableDatabase &db, int8_t &err);
docid replace_document(WritableDatabase &db, rust::Str unique_term, Document &doc,  int8_t &err);
void delete_document(WritableDatabase &db, rust::Str unique_term, Document &doc,  int8_t &err);

//
std::unique_ptr<TermGenerator> new_termgenerator(int8_t &err);
void set_stemmer (TermGenerator &tg, Stem &stem, int8_t &err);
void set_document (TermGenerator &tg, Document &doc, int8_t &err);
void index_text (TermGenerator &tg, rust::Str data, rust::Str prefix, int8_t &err);
void index_int (TermGenerator &tg, int32_t data, rust::Str prefix, int8_t &err);
void index_long (TermGenerator &tg, int64_t data, rust::Str prefix, int8_t &err);
void index_float(TermGenerator &tg, float in_data, rust::Str prefix, int8_t &err);
void index_double (TermGenerator &tg, double data, rust::Str prefix, int8_t &err);

//
std::unique_ptr<Document> new_document (int8_t &err);
void add_string (Document &doc, valueno slot, rust::Str data, int8_t &err);
void add_int (Document &doc, valueno slot, int data, int8_t &err);
void add_float(Document &doc, valueno slot, float in_data, int8_t &err);
void add_double(Document &doc, valueno slot, double in_data, int8_t &err);
void set_data (Document &doc, rust::Str data, int8_t &err);

