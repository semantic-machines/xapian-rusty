#include "xapian-bind.h"
#include <cstdint>
#include <memory>
#include <string>

#pragma once

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

} // namespace example
} // namespace org
