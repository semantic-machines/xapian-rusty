#include "xapian-bind.h"
#include <cstdint>
#include <memory>
#include <string>
#include <utility>

namespace rust {
inline namespace cxxbridge03 {
// #include "rust/cxx.h"

#ifndef CXXBRIDGE03_RUST_STR
#define CXXBRIDGE03_RUST_STR
class Str final {
public:
  Str() noexcept;
  Str(const Str &) noexcept;

  Str(const std::string &);
  Str(const char *);
  Str(std::string &&) = delete;

  Str &operator=(Str) noexcept;

  explicit operator std::string() const;

  const char *data() const noexcept;
  size_t size() const noexcept;
  size_t length() const noexcept;

  struct Repr {
    const char *ptr;
    size_t len;
  };
  Str(Repr) noexcept;
  explicit operator Repr() noexcept;

private:
  Repr repr;
};
#endif // CXXBRIDGE03_RUST_STR
} // namespace cxxbridge03
} // namespace rust

namespace org {
namespace example {

using Database = Database;
using Stem = Stem;
using WritableDatabase = WritableDatabase;
using TermGenerator = TermGenerator;
using Document = Document;

extern "C" {
Database *org$example$cxxbridge03$new_database(int8_t &err) noexcept {
  ::std::unique_ptr<Database> (*new_database$)(int8_t &) = new_database;
  return new_database$(err).release();
}

Database *org$example$cxxbridge03$new_database_with_path(::rust::Str::Repr path, int8_t db_type, int8_t &err) noexcept {
  ::std::unique_ptr<Database> (*new_database_with_path$)(::rust::Str, int8_t, int8_t &) = new_database_with_path;
  return new_database_with_path$(path, db_type, err).release();
}

void org$example$cxxbridge03$database_reopen(Database &db, int8_t &err) noexcept {
  void (*database_reopen$)(Database &, int8_t &) = database_reopen;
  database_reopen$(db, err);
}

Stem *org$example$cxxbridge03$new_stem(::rust::Str::Repr lang, int8_t &err) noexcept {
  ::std::unique_ptr<Stem> (*new_stem$)(::rust::Str, int8_t &) = new_stem;
  return new_stem$(lang, err).release();
}

WritableDatabase *org$example$cxxbridge03$new_writable_database_with_path(::rust::Str::Repr path, int8_t action, int8_t db_type, int8_t &err) noexcept {
  ::std::unique_ptr<WritableDatabase> (*new_writable_database_with_path$)(::rust::Str, int8_t, int8_t, int8_t &) = new_writable_database_with_path;
  return new_writable_database_with_path$(path, action, db_type, err).release();
}

void org$example$cxxbridge03$commit(WritableDatabase &db, int8_t &err) noexcept {
  void (*commit$)(WritableDatabase &, int8_t &) = commit;
  commit$(db, err);
}

uint32_t org$example$cxxbridge03$replace_document(WritableDatabase &db, ::rust::Str::Repr unique_term, Document &doc, int8_t &err) noexcept {
  uint32_t (*replace_document$)(WritableDatabase &, ::rust::Str, Document &, int8_t &) = replace_document;
  return replace_document$(db, unique_term, doc, err);
}

void org$example$cxxbridge03$delete_document(WritableDatabase &db, ::rust::Str::Repr unique_term, Document &doc, int8_t &err) noexcept {
  void (*delete_document$)(WritableDatabase &, ::rust::Str, Document &, int8_t &) = delete_document;
  delete_document$(db, unique_term, doc, err);
}

TermGenerator *org$example$cxxbridge03$new_termgenerator(int8_t &err) noexcept {
  ::std::unique_ptr<TermGenerator> (*new_termgenerator$)(int8_t &) = new_termgenerator;
  return new_termgenerator$(err).release();
}

void org$example$cxxbridge03$set_stemmer(TermGenerator &tg, Stem &stem, int8_t &err) noexcept {
  void (*set_stemmer$)(TermGenerator &, Stem &, int8_t &) = set_stemmer;
  set_stemmer$(tg, stem, err);
}

void org$example$cxxbridge03$set_document(TermGenerator &tg, Document &doc, int8_t &err) noexcept {
  void (*set_document$)(TermGenerator &, Document &, int8_t &) = set_document;
  set_document$(tg, doc, err);
}

void org$example$cxxbridge03$index_text(TermGenerator &tg, ::rust::Str::Repr data, int8_t &err) noexcept {
  void (*index_text$)(TermGenerator &, ::rust::Str, int8_t &) = index_text;
  index_text$(tg, data, err);
}

void org$example$cxxbridge03$index_int(TermGenerator &tg, int32_t data, int8_t &err) noexcept {
  void (*index_int$)(TermGenerator &, int32_t, int8_t &) = index_int;
  index_int$(tg, data, err);
}

void org$example$cxxbridge03$index_long(TermGenerator &tg, int64_t data, int8_t &err) noexcept {
  void (*index_long$)(TermGenerator &, int64_t, int8_t &) = index_long;
  index_long$(tg, data, err);
}

void org$example$cxxbridge03$index_float(TermGenerator &tg, float data, int8_t &err) noexcept {
  void (*index_float$)(TermGenerator &, float, int8_t &) = index_float;
  index_float$(tg, data, err);
}

void org$example$cxxbridge03$index_double(TermGenerator &tg, double data, int8_t &err) noexcept {
  void (*index_double$)(TermGenerator &, double, int8_t &) = index_double;
  index_double$(tg, data, err);
}

Document *org$example$cxxbridge03$new_document(int8_t &err) noexcept {
  ::std::unique_ptr<Document> (*new_document$)(int8_t &) = new_document;
  return new_document$(err).release();
}

void org$example$cxxbridge03$add_string(Document &doc, uint32_t slot, ::rust::Str::Repr data, int8_t &err) noexcept {
  void (*add_string$)(Document &, uint32_t, ::rust::Str, int8_t &) = add_string;
  add_string$(doc, slot, data, err);
}

void org$example$cxxbridge03$add_int(Document &doc, uint32_t slot, int32_t data, int8_t &err) noexcept {
  void (*add_int$)(Document &, uint32_t, int32_t, int8_t &) = add_int;
  add_int$(doc, slot, data, err);
}

void org$example$cxxbridge03$set_data(Document &doc, ::rust::Str::Repr data, int8_t &err) noexcept {
  void (*set_data$)(Document &, ::rust::Str, int8_t &) = set_data;
  set_data$(doc, data, err);
}
} // extern "C"

} // namespace example
} // namespace org

extern "C" {
#ifndef CXXBRIDGE03_UNIQUE_PTR_org$example$Database
#define CXXBRIDGE03_UNIQUE_PTR_org$example$Database
static_assert(sizeof(::std::unique_ptr<org::example::Database>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<org::example::Database>) == alignof(void *), "");
void cxxbridge03$unique_ptr$org$example$Database$null(::std::unique_ptr<org::example::Database> *ptr) noexcept {
  new (ptr) ::std::unique_ptr<org::example::Database>();
}
void cxxbridge03$unique_ptr$org$example$Database$raw(::std::unique_ptr<org::example::Database> *ptr, org::example::Database *raw) noexcept {
  new (ptr) ::std::unique_ptr<org::example::Database>(raw);
}
const org::example::Database *cxxbridge03$unique_ptr$org$example$Database$get(const ::std::unique_ptr<org::example::Database>& ptr) noexcept {
  return ptr.get();
}
org::example::Database *cxxbridge03$unique_ptr$org$example$Database$release(::std::unique_ptr<org::example::Database>& ptr) noexcept {
  return ptr.release();
}
void cxxbridge03$unique_ptr$org$example$Database$drop(::std::unique_ptr<org::example::Database> *ptr) noexcept {
  ptr->~unique_ptr();
}
#endif // CXXBRIDGE03_UNIQUE_PTR_org$example$Database

#ifndef CXXBRIDGE03_UNIQUE_PTR_org$example$Stem
#define CXXBRIDGE03_UNIQUE_PTR_org$example$Stem
static_assert(sizeof(::std::unique_ptr<org::example::Stem>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<org::example::Stem>) == alignof(void *), "");
void cxxbridge03$unique_ptr$org$example$Stem$null(::std::unique_ptr<org::example::Stem> *ptr) noexcept {
  new (ptr) ::std::unique_ptr<org::example::Stem>();
}
void cxxbridge03$unique_ptr$org$example$Stem$raw(::std::unique_ptr<org::example::Stem> *ptr, org::example::Stem *raw) noexcept {
  new (ptr) ::std::unique_ptr<org::example::Stem>(raw);
}
const org::example::Stem *cxxbridge03$unique_ptr$org$example$Stem$get(const ::std::unique_ptr<org::example::Stem>& ptr) noexcept {
  return ptr.get();
}
org::example::Stem *cxxbridge03$unique_ptr$org$example$Stem$release(::std::unique_ptr<org::example::Stem>& ptr) noexcept {
  return ptr.release();
}
void cxxbridge03$unique_ptr$org$example$Stem$drop(::std::unique_ptr<org::example::Stem> *ptr) noexcept {
  ptr->~unique_ptr();
}
#endif // CXXBRIDGE03_UNIQUE_PTR_org$example$Stem

#ifndef CXXBRIDGE03_UNIQUE_PTR_org$example$WritableDatabase
#define CXXBRIDGE03_UNIQUE_PTR_org$example$WritableDatabase
static_assert(sizeof(::std::unique_ptr<org::example::WritableDatabase>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<org::example::WritableDatabase>) == alignof(void *), "");
void cxxbridge03$unique_ptr$org$example$WritableDatabase$null(::std::unique_ptr<org::example::WritableDatabase> *ptr) noexcept {
  new (ptr) ::std::unique_ptr<org::example::WritableDatabase>();
}
void cxxbridge03$unique_ptr$org$example$WritableDatabase$raw(::std::unique_ptr<org::example::WritableDatabase> *ptr, org::example::WritableDatabase *raw) noexcept {
  new (ptr) ::std::unique_ptr<org::example::WritableDatabase>(raw);
}
const org::example::WritableDatabase *cxxbridge03$unique_ptr$org$example$WritableDatabase$get(const ::std::unique_ptr<org::example::WritableDatabase>& ptr) noexcept {
  return ptr.get();
}
org::example::WritableDatabase *cxxbridge03$unique_ptr$org$example$WritableDatabase$release(::std::unique_ptr<org::example::WritableDatabase>& ptr) noexcept {
  return ptr.release();
}
void cxxbridge03$unique_ptr$org$example$WritableDatabase$drop(::std::unique_ptr<org::example::WritableDatabase> *ptr) noexcept {
  ptr->~unique_ptr();
}
#endif // CXXBRIDGE03_UNIQUE_PTR_org$example$WritableDatabase

#ifndef CXXBRIDGE03_UNIQUE_PTR_org$example$TermGenerator
#define CXXBRIDGE03_UNIQUE_PTR_org$example$TermGenerator
static_assert(sizeof(::std::unique_ptr<org::example::TermGenerator>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<org::example::TermGenerator>) == alignof(void *), "");
void cxxbridge03$unique_ptr$org$example$TermGenerator$null(::std::unique_ptr<org::example::TermGenerator> *ptr) noexcept {
  new (ptr) ::std::unique_ptr<org::example::TermGenerator>();
}
void cxxbridge03$unique_ptr$org$example$TermGenerator$raw(::std::unique_ptr<org::example::TermGenerator> *ptr, org::example::TermGenerator *raw) noexcept {
  new (ptr) ::std::unique_ptr<org::example::TermGenerator>(raw);
}
const org::example::TermGenerator *cxxbridge03$unique_ptr$org$example$TermGenerator$get(const ::std::unique_ptr<org::example::TermGenerator>& ptr) noexcept {
  return ptr.get();
}
org::example::TermGenerator *cxxbridge03$unique_ptr$org$example$TermGenerator$release(::std::unique_ptr<org::example::TermGenerator>& ptr) noexcept {
  return ptr.release();
}
void cxxbridge03$unique_ptr$org$example$TermGenerator$drop(::std::unique_ptr<org::example::TermGenerator> *ptr) noexcept {
  ptr->~unique_ptr();
}
#endif // CXXBRIDGE03_UNIQUE_PTR_org$example$TermGenerator

#ifndef CXXBRIDGE03_UNIQUE_PTR_org$example$Document
#define CXXBRIDGE03_UNIQUE_PTR_org$example$Document
static_assert(sizeof(::std::unique_ptr<org::example::Document>) == sizeof(void *), "");
static_assert(alignof(::std::unique_ptr<org::example::Document>) == alignof(void *), "");
void cxxbridge03$unique_ptr$org$example$Document$null(::std::unique_ptr<org::example::Document> *ptr) noexcept {
  new (ptr) ::std::unique_ptr<org::example::Document>();
}
void cxxbridge03$unique_ptr$org$example$Document$raw(::std::unique_ptr<org::example::Document> *ptr, org::example::Document *raw) noexcept {
  new (ptr) ::std::unique_ptr<org::example::Document>(raw);
}
const org::example::Document *cxxbridge03$unique_ptr$org$example$Document$get(const ::std::unique_ptr<org::example::Document>& ptr) noexcept {
  return ptr.get();
}
org::example::Document *cxxbridge03$unique_ptr$org$example$Document$release(::std::unique_ptr<org::example::Document>& ptr) noexcept {
  return ptr.release();
}
void cxxbridge03$unique_ptr$org$example$Document$drop(::std::unique_ptr<org::example::Document> *ptr) noexcept {
  ptr->~unique_ptr();
}
#endif // CXXBRIDGE03_UNIQUE_PTR_org$example$Document
} // extern "C"
