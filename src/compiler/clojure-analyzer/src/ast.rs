//! Backend-oriented AST, primitive table, and closure-related structures
//! produced by [`crate::analyze`]. Contains no source spans; user-facing
//! diagnostics are emitted while `analysis` still owns the spanned forms.

/// Backend-oriented expression in the compilable subset.
///
/// Local and capture names have already been resolved to numeric slots. The AST
/// intentionally contains no source spans; all user-facing errors are emitted
/// while analysis still owns the spanned source forms.
#[derive(Debug, Clone)]
pub enum Ast {
    /// Signed 64-bit integer literal.
    Int(i64),
    /// IEEE-754 double literal (boxed at runtime).
    Float(f64),
    /// Boolean literal.
    Bool(bool),
    /// The `nil` literal.
    Nil,
    /// Owned UTF-8 string literal.
    Str(String),
    /// Keyword encoded without the leading colon.
    Keyword(String),
    /// Vector literal whose elements evaluate left to right.
    VecLit(Vec<Ast>),
    /// Set literal whose elements evaluate left to right.
    SetLit(Vec<Ast>),
    /// Map literal whose key/value pairs preserve source order.
    MapLit(Vec<(Ast, Ast)>),
    /// Read a top-level `def` global by its permanent-root index (ADR-0013).
    GlobalRef(u32),
    /// Initialize a top-level `def` global once, in source order (ADR-0013).
    DefGlobal {
        /// Permanent-root slot assigned to the namespace-qualified global.
        index: u32,
        /// Expression evaluated once to initialize the global slot.
        value: Box<Ast>,
    },
    /// Local slot in the current function or lambda frame.
    Local(u32),
    /// Captured value read from `self->freev[index]`.
    Capture(u32),
    /// Top-level function used as a value; lowering creates a zero-capture closure.
    FnRef(String),
    /// Closure construction with captures evaluated in the enclosing context.
    MakeFn {
        /// Generated top-level symbol containing the lambda body.
        lambda: String,
        /// Canonical callable arity stored in the closure header.
        arity: usize,
        /// Capture expressions in the lambda's assigned capture-slot order.
        captures: Vec<Ast>,
    },
    /// Conditional expression: test, then branch, else branch.
    If(Box<Ast>, Box<Ast>, Box<Ast>),
    /// Ordered expression sequence whose last value is returned.
    Do(Vec<Ast>),
    /// Lexical bindings followed by their body.
    Let {
        /// Local slot and initializer pairs in evaluation order.
        slots: Vec<(u32, Ast)>,
        /// Expression evaluated after all bindings are installed.
        body: Box<Ast>,
    },
    /// Recur target with mutable iteration slots.
    Loop {
        /// Local slot and initial-value pairs.
        slots: Vec<(u32, Ast)>,
        /// Loop body that may produce [`Ast::Recur`].
        body: Box<Ast>,
    },
    /// Tail transfer to the nearest loop or function method.
    Recur(Vec<Ast>),
    /// Direct call to a primitive or known top-level function.
    Call {
        /// Statically selected target.
        callee: Callee,
        /// Eager argument expressions in call order.
        args: Vec<Ast>,
    },
    /// Indirect invocation of a first-class callable value.
    CallValue {
        /// Expression producing the callable.
        f: Box<Ast>,
        /// Eager argument expressions.
        args: Vec<Ast>,
    },
    /// `(apply f a b ... coll)` with explicit fixed and spread arguments.
    Apply {
        /// Expression producing the callable.
        f: Box<Ast>,
        /// Arguments preceding the final collection.
        fixed: Vec<Ast>,
        /// Collection whose elements complete the argument vector.
        coll: Box<Ast>,
    },
    /// Record construction from a type name and field expressions.
    MakeRecord {
        /// Declared record type.
        type_name: String,
        /// Field names and values in declaration order.
        fields: Vec<(String, Ast)>,
    },
    /// Runtime registration of a protocol or multimethod implementation.
    RegisterMethod {
        /// Analyzer-assigned dispatch table identifier.
        method_id: i64,
        /// Dispatch key expression.
        key: Box<Ast>,
        /// Closure implementing the method.
        impl_fn: Box<Ast>,
    },
    /// Runtime registration of a multimethod dispatch function.
    RegisterMulti {
        /// Analyzer-assigned multimethod identifier.
        method_id: i64,
        /// Function applied to invocation arguments to obtain the dispatch key.
        dispatch_fn: Box<Ast>,
    },
}

/// Statically resolved target of a direct call.
#[derive(Debug, Clone)]
pub enum Callee {
    /// Built-in operation lowered through a known runtime or fast path.
    Prim(Prim),
    /// Built-in operation whose operands are proven fixnums by optimization IR.
    ///
    /// The analyzer never emits this marker. The optional optimization adapter
    /// may add it after proving every operand representation; code generation
    /// must retain overflow behavior even though type guards are redundant.
    ProvenFixnumPrim(Prim),
    /// Top-level function symbol.
    Fn(String),
}

/// Built-in operation recognized by semantic analysis.
///
/// Variants are an internal Rust/backend contract. `clojure-codegen` maps each
/// operation to inline lowering, a C ABI symbol, or a synthesized control-flow
/// sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prim {
    /// Numeric addition.
    Add,
    /// Numeric subtraction.
    Sub,
    /// Numeric multiplication.
    Mul,
    /// Truncating integer quotient.
    Quot,
    /// Euclidean integer remainder.
    Mod,
    /// Increment an integer.
    Inc,
    /// Decrement an integer.
    Dec,
    /// Structural equality.
    Eq,
    /// Numeric less-than comparison.
    Lt,
    /// Numeric less-than-or-equal comparison.
    Le,
    /// Numeric greater-than comparison.
    Gt,
    /// Numeric greater-than-or-equal comparison.
    Ge,
    /// Clojure logical negation.
    Not,
    /// Test for `nil`.
    NilP,
    /// Test whether a supported collection is empty.
    EmptyP,
    /// Prepend an item to a sequence.
    Cons,
    /// Return the first sequence item.
    First,
    /// Return the sequence after its first item.
    Rest,
    /// Count a supported collection.
    Count,
    /// Construct a list.
    List,
    /// Concatenate printable values as a string.
    Str,
    /// Print values followed by a newline.
    Println,
    /// Print values without a newline.
    Print,
    /// Print values in read syntax without a newline.
    Pr,
    /// Print values in read syntax followed by a newline.
    Prn,
    /// Write a single newline to the current output writer.
    Newline,
    /// Perform associative lookup.
    Get,
    /// Perform indexed lookup.
    Nth,
    /// Associate one or more key/value pairs.
    Assoc,
    /// Remove an associative key.
    Dissoc,
    /// Test for an associative key or set member.
    Contains,
    /// Return map keys.
    Keys,
    /// Return map values.
    Vals,
    /// Add an item to a persistent collection.
    Conj,
    /// Construct a vector.
    Vector,
    /// Construct a hash map.
    HashMap,
    /// Construct a hash set.
    HashSet,
    /// Construct a sorted map.
    SortedMap,
    /// Construct a sorted set.
    SortedSet,
    /// Compare two supported values.
    Compare,
    /// Throw a native exception value.
    Throw,
    /// Synthesized try/catch/finally operation.
    Try,
    /// Convert a persistent collection to a transient.
    Transient,
    /// Freeze a transient collection.
    PersistentBang,
    /// Add an item to a transient collection.
    ConjBang,
    /// Associate a key/value pair in a transient collection.
    AssocBang,
    /// Remove a key from a transient collection.
    DissocBang,
    /// Read a UTF-8 file as a string.
    Slurp,
    /// Write a string to a file.
    Spit,
    /// Test whether a filesystem path exists.
    FileExists,
    /// Read a process environment variable.
    Getenv,
    /// Synthesized output capture operation.
    WithOutStr,
    /// Read a built-in dynamic Var.
    VarGet,
    /// Invoke a thunk with one built-in dynamic Var rebound.
    WithBinding,
    /// Read one line from the current input stream.
    ReadLine,
    /// Create an in-memory string reader.
    StringReader,
    /// Create an in-memory string writer.
    StringWriter,
    /// Return the accumulated text of a string writer.
    WriterToString,
    /// Return nil for a non-stream, else the stream's closed flag.
    StreamClosed,
    /// Predicate: value is a reader.
    StreamReaderP,
    /// Predicate: value is a writer.
    StreamWriterP,
    /// Predicate: value is a closeable file/string stream.
    CloseableP,
    /// Read one character from an explicit reader handle.
    ReadCharFrom,
    /// Read one line from an explicit reader handle.
    ReadLineFrom,
    /// Push one character back into an explicit reader handle.
    UnreadCharTo,
    /// Write a string to an explicit writer handle.
    WriteTo,
    /// Flush an explicit writer handle.
    FlushWriter,
    /// Convert an integer to a character.
    CharOf,
    /// Convert a character to its integer code point.
    IntOf,
    /// Test whether a value is a character.
    CharP,
    /// Read one character from the current input stream.
    ReadChar,
    /// Join two path components.
    PathJoin,
    /// Return the final component of a path.
    FileName,
    /// Return the parent of a path.
    Parent,
    /// Encode a string as bytes.
    Bytes,
    /// Decode bytes as a string.
    BytesToString,
    /// Read one byte by index.
    Bget,
    /// Builds an immutable byte array from a vector of 0..255 fixnums.
    BytesOfVec,
    /// Converts an immutable byte array to a vector of 0..255 fixnums.
    BytesToVec,
    /// Returns whether an immutable byte array holds well-formed UTF-8.
    ValidUtf8,
    /// Byte input stream over an immutable byte array.
    ByteInputStream,
    /// Byte output stream accumulating bytes.
    ByteOutputStream,
    /// Read up to n bytes from a byte-input stream.
    ReadBytes,
    /// Write a byte array to a byte-output stream.
    WriteBytes,
    /// Return the accumulated bytes of a byte-output stream.
    OutputBytes,
    /// Read a block of items from a reader.
    ReadBlock,
    /// Predicate: value is a byte-input stream.
    ByteInputP,
    /// Predicate: value is a byte-output stream.
    ByteOutputP,
    /// Seek a file reader to an absolute offset.
    SeekFile,
    /// Truncate a file writer to a length.
    TruncateFile,
    /// Return the byte position of a file reader.
    PositionFile,
    /// Predicate: value is a file-backed reader.
    FileReaderP,
    /// Predicate: value is a file-backed writer.
    FileWriterP,
    /// Create a symbolic link.
    CreateSymlink,
    /// Read a symbolic link's target.
    ReadLink,
    /// Predicate: path names a symbolic link.
    NativeSymlinkP,
    /// Predicate: path string is absolute (leading '/').
    PathAbsolute,
    /// Lexically normalize a path string.
    PathNormalize,
    /// Resolve a path to its canonical form (realpath).
    RealPath,
    /// Process working directory as a string.
    ProcessCwd,
    /// Snapshot of the process environment as a map.
    ProcessEnvironment,
    /// Read a file as bytes.
    SlurpBytes,
    /// Write bytes to a file.
    SpitBytes,
    /// Parse one string at native runtime.
    ReadString,
    /// Read one form from a string reader, advancing its position.
    ReadFrom,
    /// Predicate: a string reader has only whitespace left.
    ReaderEof,
    /// Open a file-backed writer.
    WriterOpen,
    /// Open a file-backed reader.
    ReaderOpen,
    /// Close a closeable stream.
    Close,
    /// Flush the current output stream.
    Flush,
    /// Create one directory.
    Mkdir,
    /// Create a directory hierarchy.
    Mkdirs,
    /// List directory entries.
    ListDir,
    /// Delete a file.
    DeleteFile,
    /// Rename a filesystem entry.
    Rename,
    /// Test whether a path names a directory.
    DirectoryP,
    /// Test whether a path names a regular file.
    FileP,
    /// File size in bytes.
    FileSize,
    /// File last-modified time in seconds.
    FileModified,
    /// Division (`/`): exact fixnum quotient when divisible, otherwise a double.
    Div,
    /// Test whether a value is a boxed float.
    FloatP,
    /// Coerce a number to a boxed double.
    DoubleOf,
    /// Test whether a value is a string.
    StringP,
    /// Test whether a value is a fixnum integer.
    IntP,
    /// Test whether a value is a keyword.
    KeywordP,
    /// Test whether a value is a vector.
    VectorP,
    /// Test whether a value is a map.
    MapP,
    /// Test whether a value is a byte array.
    BytesP,
    /// Split a string on an ASCII separator char into a vector of strings.
    StrSplit,
    /// Parse raw HTTP request bytes into a request map (ADR-0013 Gate 4).
    ParseHttpRequest,
    /// Serialize a response map into raw HTTP/1.1 response bytes (ADR-0013 Gate 4).
    SerializeHttpResponse,
    /// Open a loopback HTTP server on a port (ADR-0013 Gate 4).
    HttpServerOpen,
    /// The bound port of an HTTP server handle.
    HttpServerPort,
    /// Block for and parse one request from an HTTP server handle.
    HttpServerAccept,
    /// Serialize/write a response and close the current connection.
    HttpServerRespond,
    /// Close an HTTP server handle (idempotent).
    HttpServerClose,
    /// Request a graceful stop of the HTTP service loop.
    HttpServerStop,
}

/// Maps built-in dynamic Vars to the C runtime's stable IDs.
///
/// ABI: values must match `enum DynVarId` in `runtime/85_writers.c`.
pub(crate) fn dyn_var_id(name: &str) -> Option<i64> {
    match name {
        "*out*" => Some(0),
        "*err*" => Some(1),
        "*flush-on-newline*" => Some(2),
        "*in*" => Some(3),
        "*command-line-args*" => Some(4),
        _ => None,
    }
}

/// One fixed or variadic arity of a function.
#[derive(Debug, Clone)]
pub struct FnMethod {
    /// Fixed parameter names in source order.
    pub params: Vec<String>,
    /// Optional variadic rest parameter.
    pub rest: Option<String>,
    /// Analyzed method body.
    pub body: Ast,
    /// Optimizer-only facts; the analyzer initializes this conservatively.
    pub optimization: MethodOptimization,
}

/// Proven method facts consumed only by optional native optimization.
#[derive(Debug, Clone, Default)]
pub struct MethodOptimization {
    /// Parameters proven fixnums at every non-escaping direct call site.
    pub proven_fixnum_params: Vec<bool>,
    /// Whether every normal return from the method is a fixnum.
    pub proven_fixnum_return: bool,
    /// Whether the method may use the isolated raw-fixnum direct-call ABI.
    ///
    /// The optimizer sets this only when the target does not escape, every
    /// observed direct call has the same fixed arity and proven fixnum
    /// arguments, and every normal return is a fixnum.
    pub specialized_fixnum_abi: bool,
}

impl FnMethod {
    /// Returns the parameter slots occupied by this arity.
    pub fn nslots(&self) -> usize {
        self.params.len() + self.rest.is_some() as usize
    }
}

/// Top-level code-generation unit.
#[derive(Debug, Clone)]
pub struct Function {
    /// Linkage symbol, including generated lambda names.
    pub name: String,
    /// One or more arities; at most one is variadic and it has greatest arity.
    pub methods: Vec<FnMethod>,
    /// Maximum local slots over all arities, excluding the implicit `self`.
    pub local_count: u32,
    /// Whether the function reads captures through the implicit `self`.
    pub is_lambda: bool,
    /// Dispatch strategy; dispatch stubs have no method bodies.
    pub dispatch: Dispatch,
}

/// Runtime dispatch strategy for a top-level symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dispatch {
    /// Ordinary function with directly compiled methods.
    None,
    /// Protocol dispatch via `lookup(method_id, type_key(arg0))`.
    Protocol(i64),
    /// Multimethod dispatch via a registered dispatch function and `:default`.
    Multi(i64),
}

/// Complete analyzer output consumed by native code generation.
#[derive(Debug, Clone)]
pub struct Program {
    /// Top-level definitions and generated lambda functions.
    pub functions: Vec<Function>,
    /// Initialization and top-level expressions, in source order.
    pub main_body: Vec<Ast>,
    /// Local slots required by the synthesized native entry point.
    pub main_local_count: u32,
    /// Number of top-level `def` globals; each owns a permanent GC root slot.
    pub global_count: u32,
}
