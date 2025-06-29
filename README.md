# xapian-rusty

![Rust](https://img.shields.io/badge/rust-2024-orange.svg)
![Version](https://img.shields.io/badge/version-0.0.56-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

**xapian-rusty** is Rust bindings for the [Xapian](https://xapian.org/) search engine library. This library gives you a safe and easy way to add full-text search to your Rust applications.

## 🚀 Features

- ✅ Full support for main Xapian functions
- ✅ Safe memory management with Rust ownership
- ✅ Easy API for indexing and searching documents
- ✅ Support for different data types (text, numbers, dates)
- ✅ Advanced query and sorting options
- ✅ Stemming and language support

## 📋 Requirements

### System Requirements
- **Rust**: edition 2024 or newer
- **C++ compiler**: GCC 4.9+ or Clang 3.4+
- **CMake**: 3.10 or newer (to build Xapian)
- **Operating System**: Linux, macOS, Windows (WSL)

### Dependencies
- Xapian 1.2.25 (install instructions below)

## 🛠️ Installation

### Step 1: Install Xapian

1. **Download Xapian 1.2.25** from the official website:
   ```bash
   wget https://oligarchy.co.uk/xapian/1.2.25/xapian-core-1.2.25.tar.xz
   tar -xf xapian-core-1.2.25.tar.xz
   cd xapian-core-1.2.25
   ```

2. **Apply patches** (if using patches from repository):
   ```bash
   # Copy files from xapian-patch/ to Xapian directories
   cp /path/to/xapian-rusty/xapian-patch/include/xapian/enquire.h include/xapian/
   cp /path/to/xapian-rusty/xapian-patch/api/omenquire.cc api/
   ```

3. **Build Xapian**:
   ```bash
   ./configure --prefix=/usr/local
   make -j$(nproc)
   sudo make install
   ```

4. **Rename the library** (for xapian-rusty compatibility):
   ```bash
   sudo ln -s /usr/local/lib/libxapian.so /usr/local/lib/libxapianm.so
   # or copy:
   sudo cp /usr/local/lib/libxapian.so /usr/local/lib/libxapianm.so
   ```

### Step 2: Install xapian-rusty

Add to your `Cargo.toml`:

```toml
[dependencies]
xapian-rusty = "0.0.56"
```

Or install from local repository:

```bash
git clone https://github.com/your-repo/xapian-rusty.git
cd xapian-rusty
cargo build --release
```

## 📖 Usage Examples

### Create Index and Add Documents

```rust
use xapian_rusty::*;

fn main() -> Result<(), XError> {
    // Create database for writing
    let mut db = WritableDatabase::new("./my_index", DB_CREATE_OR_OPEN, BRASS)?;
    
    // Create term generator with stemming support
    let mut stem = Stem::new("english")?;
    let mut termgen = TermGenerator::new()?;
    termgen.set_stemmer(&mut stem)?;
    
    // Create document
    let mut doc = Document::new()?;
    doc.set_data("This is a test document for indexing")?;
    doc.add_string(0, "document title")?;
    doc.add_string(1, "document description")?;
    
    // Index text
    termgen.set_document(&mut doc)?;
    termgen.index_text("This is a test document for full-text search")?;
    termgen.index_text_with_prefix("document title", "S")?;
    
    // Add document to database
    db.replace_document("doc1", &mut doc)?;
    db.commit()?;
    
    println!("Document indexed successfully!");
    Ok(())
}
```

### Search Documents

```rust
use xapian_rusty::*;

fn main() -> Result<(), XError> {
    // Open database for reading
    let mut db = Database::new_with_path("./my_index", BRASS)?;
    let mut enquire = db.new_enquire()?;
    
    // Create query parser
    let mut qp = QueryParser::new()?;
    let mut stem = Stem::new("english")?;
    qp.set_stemmer(&mut stem)?;
    qp.set_database(&mut db)?;
    
    // Parse and run query
    let mut query = qp.parse_query("test search", 
        FeatureFlag::FlagDefault as i16)?;
    
    enquire.set_query(&mut query)?;
    let mut mset = enquire.get_mset(0, 10)?;
    
    println!("Found {} documents", mset.get_matches_estimated()?);
    
    // Go through results
    let mut iter = mset.iterator()?;
    while iter.is_next()? {
        let data = iter.get_document_data()?;
        println!("Document: {}", data);
        iter.next()?;
    }
    
    Ok(())
}
```

### Advanced Search with Sorting

```rust
use xapian_rusty::*;

fn main() -> Result<(), XError> {
    let mut db = Database::new_with_path("./my_index", BRASS)?;
    let mut enquire = db.new_enquire()?;
    
    // Create complex query
    let mut query1 = Query::new_double_with_prefix("price", 100.0)?;
    let mut query2 = Query::new_range(XapianOp::OpValueRange as i32, 1, 50.0, 200.0)?;
    let combined_query = query1.add_right(XapianOp::OpAnd, &mut query2)?;
    
    // Set up sorting
    let mut sorter = MultiValueKeyMaker::new()?;
    sorter.add_value(0, true)?; // sort by field 0 in ascending order
    enquire.set_sort_by_key(sorter, false)?;
    
    enquire.set_query(&mut combined_query)?;
    let mut mset = enquire.get_mset(0, 20)?;
    
    println!("Results with sorting:");
    let mut iter = mset.iterator()?;
    while iter.is_next()? {
        println!("Document: {}", iter.get_document_data()?);
        iter.next()?;
    }
    
    Ok(())
}
```

## 🏗️ Project Structure

```
xapian-rusty/
├── src/
│   └── lib.rs              # Main library with Rust API
├── include/
│   └── xapian/             # Xapian header files
├── xapian-patch/           # Patches for Xapian
│   ├── api/
│   └── include/
├── build.rs                # Build script
├── xapian-bind.cc          # C++ bindings
├── xapian-bind.h           # Binding header files
└── Cargo.toml              # Rust configuration
```

## 🔧 Troubleshooting

### Build Errors

**Problem**: `error: could not find native library xapianm`
```bash
# Solution: make sure libxapianm.so is created
sudo ln -s /usr/local/lib/libxapian.so /usr/local/lib/libxapianm.so
# Add library path:
export LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH
```

**Problem**: Compiler warnings
```bash
# Solution: set environment variables
CC=gcc CXX=g++ cargo build --release
```

**Problem**: C++ build errors
```bash
# Make sure required tools are installed:
sudo apt-get install build-essential cmake libxapian-dev
```

### Runtime Errors

**Problem**: `DatabaseError` when opening database
- Check file permissions for database directory
- Make sure database path is correct
- Check if database is not corrupted

**Problem**: Text encoding issues
- Make sure all text is in UTF-8
- Use correct language stemmers

## 📚 API Documentation

### Main Types
- `Database` - database for reading
- `WritableDatabase` - database for writing
- `Document` - document for indexing
- `Query` - search query
- `TermGenerator` - term generator for indexing
- `QueryParser` - search query parser
- `Enquire` - interface for running queries
- `MSet` - search results

### Error Types
- `XError::Xapian(code)` - Xapian errors with code
- `XError::Io(error)` - input/output errors

## 🤝 Contributing

We welcome contributions to this project! Please:

1. Fork the repository
2. Create a branch for your changes (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Create a Pull Request

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 👥 Authors

- **Valeriy Bushenev** - [ValeriyBushenev@gmail.com](mailto:ValeriyBushenev@gmail.com)
- **David Tolnay** - [dtolnay@gmail.com](mailto:dtolnay@gmail.com)

## 🔗 Useful Links

- [Official Xapian Documentation](https://xapian.org/docs/)
- [Xapian Downloads](https://xapian.org/download)
- [Rust Book](https://doc.rust-lang.org/book/)
- [cxx - C++ interop](https://cxx.rs/)

---

**Note**: This project is under active development. The API may change between versions.



