use crate::ast::*;
use clojure_diagnostics::Diagnostic;
use clojure_span::Span;

pub(crate) fn prim_of(name: &str) -> Option<Prim> {
    Some(match name {
        "+" => Prim::Add,
        "-" => Prim::Sub,
        "*" => Prim::Mul,
        "quot" => Prim::Quot,
        "mod" => Prim::Mod,
        "inc" => Prim::Inc,
        "dec" => Prim::Dec,
        "=" => Prim::Eq,
        "<" => Prim::Lt,
        "<=" => Prim::Le,
        ">" => Prim::Gt,
        ">=" => Prim::Ge,
        "not" => Prim::Not,
        "nil?" => Prim::NilP,
        "empty?" => Prim::EmptyP,
        "cons" => Prim::Cons,
        "first" => Prim::First,
        "rest" => Prim::Rest,
        "count" => Prim::Count,
        "list" => Prim::List,
        "str" => Prim::Str,
        "println" => Prim::Println,
        "print" => Prim::Print,
        "pr" => Prim::Pr,
        "prn" => Prim::Prn,
        "newline" => Prim::Newline,
        "get" => Prim::Get,
        "nth" => Prim::Nth,
        "assoc" => Prim::Assoc,
        "dissoc" => Prim::Dissoc,
        "contains?" => Prim::Contains,
        "keys" => Prim::Keys,
        "vals" => Prim::Vals,
        "conj" => Prim::Conj,
        "vector" => Prim::Vector,
        "hash-map" => Prim::HashMap,
        "hash-set" => Prim::HashSet,
        "set" => Prim::HashSet,
        "sorted-map" => Prim::SortedMap,
        "sorted-set" => Prim::SortedSet,
        "compare" => Prim::Compare,
        "throw" => Prim::Throw,
        "transient" => Prim::Transient,
        "persistent!" => Prim::PersistentBang,
        "conj!" => Prim::ConjBang,
        "assoc!" => Prim::AssocBang,
        "dissoc!" => Prim::DissocBang,
        "slurp" => Prim::Slurp,
        "spit" => Prim::Spit,
        "file-exists?" => Prim::FileExists,
        "getenv" => Prim::Getenv,
        "read-line" => Prim::ReadLine,
        "read-char" => Prim::ReadChar,
        "char" => Prim::CharOf,
        "int" => Prim::IntOf,
        "char?" => Prim::CharP,
        "path-join" => Prim::PathJoin,
        "file-name" => Prim::FileName,
        "parent" => Prim::Parent,
        "bytes" => Prim::Bytes,
        "bytes->string" => Prim::BytesToString,
        "bget" => Prim::Bget,
        "string-reader" => Prim::StringReader,
        "string-writer" => Prim::StringWriter,
        "writer->string" => Prim::WriterToString,
        "stream-closed" => Prim::StreamClosed,
        "reader?" => Prim::StreamReaderP,
        "writer?" => Prim::StreamWriterP,
        "closeable?" => Prim::CloseableP,
        "read-char-from" => Prim::ReadCharFrom,
        "read-line-from" => Prim::ReadLineFrom,
        "unread-char-to" => Prim::UnreadCharTo,
        "write-to" => Prim::WriteTo,
        "flush-writer" => Prim::FlushWriter,
        "bytes-of-vec" => Prim::BytesOfVec,
        "bytes->vec" => Prim::BytesToVec,
        "valid-utf8?" => Prim::ValidUtf8,
        "byte-input-stream" => Prim::ByteInputStream,
        "byte-output-stream" => Prim::ByteOutputStream,
        "read-bytes" => Prim::ReadBytes,
        "write-bytes!" => Prim::WriteBytes,
        "output-bytes" => Prim::OutputBytes,
        "read-block!" => Prim::ReadBlock,
        "byte-input?" => Prim::ByteInputP,
        "byte-output?" => Prim::ByteOutputP,
        "seek-file" => Prim::SeekFile,
        "truncate-file" => Prim::TruncateFile,
        "position-file" => Prim::PositionFile,
        "file-reader?" => Prim::FileReaderP,
        "file-writer?" => Prim::FileWriterP,
        "create-symlink" => Prim::CreateSymlink,
        "read-link" => Prim::ReadLink,
        "native-symlink?" => Prim::NativeSymlinkP,
        "path-absolute" => Prim::PathAbsolute,
        "path-normalize" => Prim::PathNormalize,
        "real-path" => Prim::RealPath,
        "process-cwd" => Prim::ProcessCwd,
        "process-environment" => Prim::ProcessEnvironment,
        "slurp-bytes" => Prim::SlurpBytes,
        "spit-bytes" => Prim::SpitBytes,
        "read-string" => Prim::ReadString,
        "read-from" => Prim::ReadFrom,
        "reader-eof?" => Prim::ReaderEof,
        "writer" => Prim::WriterOpen,
        "reader" => Prim::ReaderOpen,
        "close" => Prim::Close,
        "flush" => Prim::Flush,
        "mkdir" => Prim::Mkdir,
        "mkdirs" => Prim::Mkdirs,
        "list-dir" => Prim::ListDir,
        "delete-file" => Prim::DeleteFile,
        "rename" => Prim::Rename,
        "directory?" => Prim::DirectoryP,
        "file?" => Prim::FileP,
        "file-size" => Prim::FileSize,
        "file-modified" => Prim::FileModified,
        "/" => Prim::Div,
        "float?" => Prim::FloatP,
        "double" => Prim::DoubleOf,
        "string?" => Prim::StringP,
        "int?" => Prim::IntP,
        "integer?" => Prim::IntP,
        "keyword?" => Prim::KeywordP,
        "vector?" => Prim::VectorP,
        "map?" => Prim::MapP,
        "bytes?" => Prim::BytesP,
        "str-split" => Prim::StrSplit,
        "parse-http-request" => Prim::ParseHttpRequest,
        "serialize-http-response" => Prim::SerializeHttpResponse,
        "http-server-open" => Prim::HttpServerOpen,
        "http-server-port" => Prim::HttpServerPort,
        "http-server-accept" => Prim::HttpServerAccept,
        "http-server-respond" => Prim::HttpServerRespond,
        "http-server-close" => Prim::HttpServerClose,
        "http-server-stop" => Prim::HttpServerStop,
        _ => return None,
    })
}

/// Returns the canonical arity when a primitive is used as a first-class value.
///
/// Variadic and synthesized primitives return `None`.
pub(crate) fn prim_value_arity(prim: Prim) -> Option<usize> {
    Some(match prim {
        Prim::Inc
        | Prim::Dec
        | Prim::Not
        | Prim::NilP
        | Prim::EmptyP
        | Prim::First
        | Prim::Rest
        | Prim::Count
        | Prim::Keys
        | Prim::Throw
        | Prim::Transient
        | Prim::PersistentBang
        | Prim::Slurp
        | Prim::FileExists
        | Prim::Getenv
        | Prim::CharOf
        | Prim::IntOf
        | Prim::CharP
        | Prim::FileName
        | Prim::Parent
        | Prim::Bytes
        | Prim::BytesToString
        | Prim::BytesOfVec
        | Prim::BytesToVec
        | Prim::ValidUtf8
        | Prim::SlurpBytes
        | Prim::ReadString
        | Prim::ReadFrom
        | Prim::ReaderEof
        | Prim::ParseHttpRequest
        | Prim::SerializeHttpResponse
        | Prim::HttpServerOpen
        | Prim::HttpServerPort
        | Prim::HttpServerAccept
        | Prim::HttpServerClose
        | Prim::HttpServerStop
        | Prim::WriterOpen
        | Prim::ReaderOpen
        | Prim::Close
        | Prim::Mkdir
        | Prim::Mkdirs
        | Prim::ListDir
        | Prim::DeleteFile
        | Prim::DirectoryP
        | Prim::FileP
        | Prim::FileSize
        | Prim::FileModified
        | Prim::FloatP
        | Prim::DoubleOf
        | Prim::StringP
        | Prim::IntP
        | Prim::KeywordP
        | Prim::VectorP
        | Prim::MapP
        | Prim::BytesP
        | Prim::WriterToString
        | Prim::StreamClosed
        | Prim::StreamReaderP
        | Prim::StreamWriterP
        | Prim::ReadCharFrom
        | Prim::ReadLineFrom
| Prim::FlushWriter
        | Prim::CloseableP
        | Prim::ByteInputStream
        | Prim::OutputBytes
        | Prim::ByteInputP
        | Prim::ByteOutputP
        | Prim::PositionFile
        | Prim::FileReaderP
        | Prim::FileWriterP
        | Prim::ReadLink
        | Prim::NativeSymlinkP
        | Prim::PathAbsolute
        | Prim::PathNormalize
        | Prim::RealPath
        | Prim::Vals => 1,
        Prim::ByteOutputStream => 0,
        Prim::ProcessCwd | Prim::ProcessEnvironment => 0,
        Prim::Newline => 0,
        Prim::ReadBytes | Prim::WriteBytes | Prim::ReadBlock | Prim::SeekFile | Prim::TruncateFile | Prim::CreateSymlink => 2,
        Prim::StringWriter => 0,
        Prim::UnreadCharTo | Prim::WriteTo => 2,
        Prim::Add
        | Prim::Sub
        | Prim::Mul
        | Prim::Quot
        | Prim::Mod
        | Prim::Eq
        | Prim::Lt
        | Prim::Le
        | Prim::Gt
        | Prim::Ge
        | Prim::Cons
        | Prim::Get
        | Prim::Nth
        | Prim::Dissoc
        | Prim::Contains
        | Prim::Compare
        | Prim::ConjBang
        | Prim::DissocBang
        | Prim::Spit
        | Prim::PathJoin
        | Prim::Bget
        | Prim::SpitBytes
        | Prim::Rename
        | Prim::Div
        | Prim::StrSplit
        | Prim::HttpServerRespond
        | Prim::Conj => 2,
        Prim::Assoc | Prim::AssocBang => 3,
        Prim::Str
        | Prim::List
        | Prim::Vector
        | Prim::HashMap
        | Prim::HashSet
        | Prim::SortedMap
        | Prim::SortedSet
        | Prim::Println
        | Prim::Try // sintetizada; nunca usada como valor de 1ª classe
        | Prim::WithOutStr // idem: só via forma especial
        | Prim::VarGet // sintetizada (leitura de Var dinâmica)
        | Prim::WithBinding // sintetizada (desugar de binding)
        | Prim::ReadLine // 0-ária; use (fn [] (read-line)) como valor
        | Prim::ReadChar // idem
        | Prim::Flush // 0-ária
        | Prim::StringReader // sintetizada (with-in-str)
        | Prim::Pr
        | Prim::Prn
        | Prim::Print => return None,
    })
}

pub(crate) fn check_prim_arity(prim: Prim, n: usize, span: Span) -> Result<(), Diagnostic> {
    let ok = match prim {
        Prim::Sub | Prim::Add | Prim::Mul => n >= 1,
        Prim::Quot
        | Prim::Mod
        | Prim::Cons
        | Prim::Get
        | Prim::Dissoc
        | Prim::Contains
        | Prim::ConjBang
        | Prim::DissocBang
        | Prim::Spit
        | Prim::Conj => n == 2,
        Prim::Nth => n == 2 || n == 3, // ADR-0008: aridade 2 e 3 (not-found)
        Prim::AssocBang => n == 3,
        Prim::Assoc => n >= 3 && n % 2 == 1, // coll + um ou mais pares
        Prim::Inc
        | Prim::Dec
        | Prim::Not
        | Prim::NilP
        | Prim::EmptyP
        | Prim::First
        | Prim::Rest
        | Prim::Count
        | Prim::Keys
        | Prim::Throw
        | Prim::Transient
        | Prim::PersistentBang
        | Prim::Slurp
        | Prim::FileExists
        | Prim::Getenv
        | Prim::Vals => n == 1,
        Prim::Try => n == 3,
        Prim::WithOutStr => n == 1,
        Prim::VarGet => n == 1,
        Prim::WithBinding => n == 3,
        Prim::ReadLine => n == 0,
        Prim::ReadChar => n == 0,
        Prim::StringReader => n == 1,
        Prim::StringWriter => n == 0,
        Prim::WriterToString
        | Prim::StreamClosed
        | Prim::StreamReaderP
        | Prim::StreamWriterP
        | Prim::ReadCharFrom
        | Prim::ReadLineFrom
        | Prim::FlushWriter
        | Prim::CloseableP => n == 1,
        Prim::UnreadCharTo | Prim::WriteTo => n == 2,
        Prim::CharOf | Prim::IntOf | Prim::CharP => n == 1,
        Prim::StringP | Prim::IntP | Prim::KeywordP | Prim::VectorP | Prim::MapP | Prim::BytesP => {
            n == 1
        }
        Prim::PathJoin => n == 2,
        Prim::FileName | Prim::Parent => n == 1,
        Prim::Bytes | Prim::BytesToString | Prim::SlurpBytes | Prim::ReadString => n == 1,
        Prim::ReadFrom | Prim::ReaderEof => n == 1,
        Prim::BytesOfVec | Prim::BytesToVec | Prim::ValidUtf8 => n == 1,
        Prim::ByteOutputStream => n == 0,
        Prim::ByteInputStream | Prim::OutputBytes | Prim::ByteInputP | Prim::ByteOutputP => n == 1,
        Prim::ReadBytes | Prim::WriteBytes | Prim::ReadBlock => n == 2,
        Prim::PositionFile | Prim::FileReaderP | Prim::FileWriterP => n == 1,
        Prim::ReadLink | Prim::NativeSymlinkP => n == 1,
        Prim::PathAbsolute | Prim::PathNormalize | Prim::RealPath => n == 1,
        Prim::ProcessCwd | Prim::ProcessEnvironment => n == 0,
        Prim::CreateSymlink => n == 2,
        Prim::SeekFile | Prim::TruncateFile => n == 2,
        Prim::ParseHttpRequest | Prim::SerializeHttpResponse => n == 1,
        Prim::HttpServerOpen
        | Prim::HttpServerPort
        | Prim::HttpServerAccept
        | Prim::HttpServerClose
        | Prim::HttpServerStop => n == 1,
        Prim::HttpServerRespond => n == 2,
        Prim::Bget | Prim::SpitBytes | Prim::StrSplit => n == 2,
        Prim::WriterOpen | Prim::ReaderOpen | Prim::Close => n == 1,
        Prim::Flush => n == 0,
        Prim::Mkdir
        | Prim::Mkdirs
        | Prim::ListDir
        | Prim::DeleteFile
        | Prim::DirectoryP
        | Prim::FileP
        | Prim::FileSize
        | Prim::FileModified
        | Prim::FloatP
        | Prim::DoubleOf => n == 1,
        Prim::Rename | Prim::Div => n == 2,
        Prim::Eq | Prim::Lt | Prim::Le | Prim::Gt | Prim::Ge | Prim::Compare => n == 2,
        Prim::HashMap | Prim::SortedMap => n & 1 == 0,
        Prim::Newline => n == 0,
        Prim::List
        | Prim::Str
        | Prim::Println
        | Prim::Print
        | Prim::Pr
        | Prim::Prn
        | Prim::Vector
        | Prim::HashSet
        | Prim::SortedSet => true,
    };
    if ok {
        Ok(())
    } else {
        Err(Diagnostic::error(
            "E0105",
            format!("aridade inválida para primitiva ({n} args)"),
        )
        .with_span(span))
    }
}

pub(crate) fn unsupported(msg: impl Into<String>, span: Span) -> Diagnostic {
    Diagnostic::error("E0100", msg)
        .with_span(span)
        .with_help("fora do subconjunto compilável atual; ver specs/LANGUAGE_SCOPE.md e IMPLEMENTATION_PLAN.md")
}
