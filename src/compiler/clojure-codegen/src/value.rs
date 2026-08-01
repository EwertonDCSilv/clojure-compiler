//! Static value classification used by `FnGen` to elide GC roots at
//! allocation safepoints and to fast-path primitives whose result never
//! needs boxing.

use clojure_analyzer::Prim;
use cranelift_codegen::ir::Value as CValue;

pub(crate) enum Flow {
    Val(CValue),
    Diverged,
}

/// Static classification used to elide roots at allocation safepoints.
///
/// `Imm` proves that a Value cannot be a heap pointer; `Heap` conservatively
/// retains eager rooting. GC: elision is sound because only proven immediates
/// omit roots. Unwritten frame slots remain NIL after `cljn_gc_enter`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum VKind {
    Fixnum,
    Imm,
    Heap,
}

impl VKind {
    pub(crate) fn join(self, other: VKind) -> VKind {
        match (self, other) {
            (VKind::Heap, _) | (_, VKind::Heap) => VKind::Heap,
            (VKind::Fixnum, VKind::Fixnum) => VKind::Fixnum,
            _ => VKind::Imm,
        }
    }
}

/// Tests whether every successful path returns a tagged fixnum.
pub(crate) fn prim_fixnum_result(p: Prim) -> bool {
    matches!(
        p,
        Prim::Quot
            | Prim::Mod
            | Prim::Count
            | Prim::Compare
            | Prim::IntOf
            | Prim::Bget
            | Prim::FileSize
            | Prim::FileModified
    )
}

/// Tests whether every path of a primitive returns an immediate Value.
pub(crate) fn prim_imm_result(p: Prim) -> bool {
    // Add/Sub/Mul/Inc/Dec are excluded: with floats they may return a boxed
    // (heap) double, so their results must be treated as heap-capable and rooted.
    matches!(
        p,
        Prim::Quot
            | Prim::Mod
            | Prim::Eq
            | Prim::Lt
            | Prim::Le
            | Prim::Gt
            | Prim::Ge
            | Prim::Not
            | Prim::NilP
            | Prim::EmptyP
            | Prim::Contains
            | Prim::Count
            | Prim::Compare
            | Prim::Println
            | Prim::Print
            | Prim::Throw
            | Prim::Spit
            | Prim::FileExists
            | Prim::CharOf
            | Prim::IntOf
            | Prim::CharP
            | Prim::ReadChar
            | Prim::Bget
            | Prim::ValidUtf8
            | Prim::WriteBytes
            | Prim::ByteInputP
            | Prim::ByteOutputP
            | Prim::SeekFile
            | Prim::TruncateFile
            | Prim::PositionFile
            | Prim::FileReaderP
            | Prim::FileWriterP
            | Prim::CreateSymlink
            | Prim::NativeSymlinkP
            | Prim::PathAbsolute
            | Prim::ReaderEof
            | Prim::SpitBytes
            | Prim::Close
            | Prim::Flush
            | Prim::StreamClosed
            | Prim::StreamReaderP
            | Prim::StreamWriterP
            | Prim::ReadCharFrom
            | Prim::UnreadCharTo
            | Prim::WriteTo
            | Prim::FlushWriter
            | Prim::CloseableP
            | Prim::Mkdir
            | Prim::Mkdirs
            | Prim::DeleteFile
            | Prim::Rename
            | Prim::DirectoryP
            | Prim::FileP
            | Prim::FileSize
            | Prim::FileModified
            | Prim::FloatP
            | Prim::StringP
            | Prim::IntP
            | Prim::KeywordP
            | Prim::VectorP
            | Prim::MapP
            | Prim::BytesP
            | Prim::HttpServerPort // devolve fixnum
            | Prim::HttpServerRespond // devolve nil
            | Prim::HttpServerClose // devolve nil
            | Prim::HttpServerStop // devolve nil
    )
}
