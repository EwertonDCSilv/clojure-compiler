//! Cranelift import declarations for the C runtime ABI: one [`Runtime`]
//! field per `cljn_*` entry point, declared once per compiled object by
//! [`declare_runtime`]. Signatures here must match the corresponding
//! `runtime/*.c` definitions exactly; nothing in this module changes them.

use cranelift_codegen::ir::{types, AbiParam};
use cranelift_module::{DataId, FuncId, Linkage, Module};
use cranelift_object::ObjectModule;

/// Cranelift imports for the C runtime ABI.
pub(crate) struct Runtime {
    // Binary (Value, Value) -> Value operations.
    pub(crate) add: FuncId,
    pub(crate) sub: FuncId,
    pub(crate) mul: FuncId,
    pub(crate) quot: FuncId,
    pub(crate) mod_: FuncId,
    pub(crate) lt: FuncId,
    pub(crate) le: FuncId,
    pub(crate) gt: FuncId,
    pub(crate) ge: FuncId,
    pub(crate) eq: FuncId,
    pub(crate) cons: FuncId,
    pub(crate) str_concat: FuncId,
    // Unary Value -> Value operations.
    pub(crate) inc: FuncId,
    pub(crate) dec: FuncId,
    pub(crate) not_: FuncId,
    pub(crate) nilp: FuncId,
    pub(crate) emptyp: FuncId,
    pub(crate) first: FuncId,
    pub(crate) rest: FuncId,
    pub(crate) count: FuncId,
    pub(crate) to_str: FuncId,
    // Constructors, predicates, and output.
    pub(crate) empty: FuncId,       // ()->i64
    pub(crate) str_from: FuncId,    // (ptr,i64)->i64
    pub(crate) truthy: FuncId,      // (i64)->i32
    pub(crate) print: FuncId,       // (i64)->void
    pub(crate) print_space: FuncId, // ()->void
    pub(crate) print_newline: FuncId,
    pub(crate) out_check: FuncId,   // ()->void, throws on closed *out*
    pub(crate) out_newline: FuncId, // ()->void, checked newline
    // GC shadow-stack frame operations.
    pub(crate) gc_enter: FuncId, // (slot_count) -> base.
    pub(crate) gc_leave: FuncId, // (base) -> void.
    // GC: push/pop/set lower to direct stores in these exported globals.
    pub(crate) gc_stack_data: DataId,    // Value gc_stack[]
    pub(crate) gc_sp_data: DataId,       // int64_t gc_sp
    pub(crate) const_cache_data: DataId, // Value cljn_const_cache[]
    pub(crate) const_register: FuncId,   // (id,v)->void
    // First-class function objects.
    pub(crate) make_fn: FuncId,  // (code,arity,nfree)->fn
    pub(crate) set_free: FuncId, // (fn,i,v)->void
    pub(crate) fn_free: FuncId,  // (fn,i)->v
    pub(crate) fn_code: FuncId,  // (fn)->code
    pub(crate) check_fn: FuncId, // (fn)->void  (é função?)
    // ABI: uniform entry convention is (self, argc, argv).
    pub(crate) argv: FuncId,         // (argc)->ptr (topo do shadow-stack)
    pub(crate) check_arity: FuncId,  // (argc,expected)->void
    pub(crate) collect_rest: FuncId, // (argc,argv,nfixed)->list
    pub(crate) spread_args: FuncId,  // (fixed_argc,coll)->argc_total
    // Collections and runtime facilities.
    pub(crate) kw: FuncId,                      // (ptr,len)->kw
    pub(crate) vec_empty: FuncId,               // ()->vec
    pub(crate) vec_conj: FuncId,                // (vec,x)->vec
    pub(crate) set_alloc: FuncId,               // (n)->set
    pub(crate) set_add: FuncId,                 // (set,x)->void
    pub(crate) map_alloc: FuncId,               // (n)->map
    pub(crate) map_set: FuncId,                 // (map,i,k,v)->void
    pub(crate) p_get: FuncId,                   // (coll,key)->v
    pub(crate) p_nth: FuncId,                   // (coll,i)->v   (nth aridade 2)
    pub(crate) p_nth_or: FuncId,                // (coll,i,nf)->v (nth aridade 3)
    pub(crate) p_assoc: FuncId,                 // (coll,k,v)->coll
    pub(crate) p_dissoc: FuncId,                // (map,k)->map
    pub(crate) p_contains: FuncId,              // (coll,key)->bool
    pub(crate) p_keys: FuncId,                  // (map)->list
    pub(crate) p_vals: FuncId,                  // (map)->list
    pub(crate) p_conj: FuncId,                  // (coll,x)->coll
    pub(crate) sorted_map_empty: FuncId,        // ()->sorted-map
    pub(crate) sorted_set_empty: FuncId,        // ()->sorted-set
    pub(crate) sorted_assoc: FuncId,            // (smap,k,v)->smap
    pub(crate) compare: FuncId,                 // (a,b)->fixnum(-1/0/1)
    pub(crate) try_: FuncId,                    // (body_fn,catch_fn|nil,finally_fn|nil)->v
    pub(crate) throw_: FuncId,                  // (v)->! (longjmp)
    pub(crate) multi_register: FuncId,          // (mid,dispatch_fn)->void
    pub(crate) multi_call: FuncId,              // (mid,argc,argv)->v
    pub(crate) slurp: FuncId,                   // (path)->string
    pub(crate) spit: FuncId,                    // (path,content)->nil
    pub(crate) file_exists: FuncId,             // (path)->bool
    pub(crate) getenv: FuncId,                  // (name)->string|nil
    pub(crate) with_out_str: FuncId,            // (thunk)->string
    pub(crate) var_get: FuncId,                 // (id) -> dynamic Var value.
    pub(crate) with_binding: FuncId,            // (id, value, thunk) -> body result.
    pub(crate) read_line: FuncId,               // () -> string|nil from *in*.
    pub(crate) string_reader: FuncId,           // (string) -> reader.
    pub(crate) string_writer: FuncId,           // () -> writer.
    pub(crate) writer_to_string: FuncId,        // (writer) -> string.
    pub(crate) stream_closed: FuncId,           // (x) -> nil|bool.
    pub(crate) reader_p: FuncId,                // (x) -> bool.
    pub(crate) writer_p: FuncId,                // (x) -> bool.
    pub(crate) read_char_from: FuncId,          // (reader) -> char|nil.
    pub(crate) read_line_from: FuncId,          // (reader) -> string|nil.
    pub(crate) unread_char_to: FuncId,          // (reader,char) -> nil.
    pub(crate) write_to: FuncId,                // (writer,string) -> nil.
    pub(crate) flush_writer: FuncId,            // (writer) -> nil.
    pub(crate) closeable_p: FuncId,             // (x) -> bool.
    pub(crate) char_of: FuncId,                 // (int|char)->char
    pub(crate) int_of: FuncId,                  // (char|int)->int
    pub(crate) charp: FuncId,                   // (x)->bool
    pub(crate) read_char: FuncId,               // ()->char|nil
    pub(crate) path_join: FuncId,               // (a,b)->string
    pub(crate) file_name: FuncId,               // (p)->string
    pub(crate) parent: FuncId,                  // (p)->string|nil
    pub(crate) bytes: FuncId,                   // (string)->bytes
    pub(crate) bytes_to_string: FuncId,         // (bytes)->string
    pub(crate) bget: FuncId,                    // (bytes,i)->fixnum
    pub(crate) bytes_of_vec: FuncId,            // (vec)->bytes
    pub(crate) bytes_to_vec: FuncId,            // (bytes)->vec
    pub(crate) valid_utf8: FuncId,              // (bytes)->bool
    pub(crate) byte_input_stream: FuncId,       // (bytes)->reader
    pub(crate) byte_output_stream: FuncId,      // ()->writer
    pub(crate) read_bytes: FuncId,              // (reader,n)->bytes
    pub(crate) write_bytes: FuncId,             // (writer,bytes)->nil
    pub(crate) output_bytes: FuncId,            // (writer)->bytes
    pub(crate) read_block: FuncId,              // (reader,n)->string|bytes
    pub(crate) byte_input_p: FuncId,            // (x)->bool
    pub(crate) byte_output_p: FuncId,           // (x)->bool
    pub(crate) seek_file: FuncId,               // (reader,n)->nil
    pub(crate) truncate_file: FuncId,           // (writer,n)->nil
    pub(crate) position_file: FuncId,           // (reader)->fixnum
    pub(crate) file_reader_p: FuncId,           // (x)->bool
    pub(crate) file_writer_p: FuncId,           // (x)->bool
    pub(crate) create_symlink: FuncId,          // (target,link)->nil
    pub(crate) read_link: FuncId,               // (path)->string
    pub(crate) native_symlink_p: FuncId,        // (x)->bool
    pub(crate) path_absolute: FuncId,           // (path)->bool
    pub(crate) path_normalize: FuncId,          // (path)->string
    pub(crate) real_path: FuncId,               // (path)->string
    pub(crate) process_cwd: FuncId,             // ()->string
    pub(crate) process_environment: FuncId,     // ()->map
    pub(crate) slurp_bytes: FuncId,             // (path)->bytes
    pub(crate) spit_bytes: FuncId,              // (path,bytes)->nil
    pub(crate) read_string: FuncId,             // (string) -> parsed EDN value
    pub(crate) read_from: FuncId,               // (reader) -> parsed form
    pub(crate) reader_eof: FuncId,              // (reader) -> bool
    pub(crate) set_args: FuncId,                // (raw argc, raw argv) -> void
    pub(crate) writer_open: FuncId,             // (path) -> file writer.
    pub(crate) reader_open: FuncId,             // (path) -> file reader.
    pub(crate) close: FuncId,                   // (closeable) -> nil.
    pub(crate) flush: FuncId,                   // () -> nil; flushes *out*.
    pub(crate) mkdir: FuncId,                   // (path)->nil
    pub(crate) mkdirs: FuncId,                  // (path) -> nil, recursively.
    pub(crate) list_dir: FuncId,                // (path) -> vector of names.
    pub(crate) delete_file: FuncId,             // (path)->nil
    pub(crate) rename: FuncId,                  // (from,to)->nil
    pub(crate) directoryp: FuncId,              // (path)->bool
    pub(crate) filep: FuncId,                   // (path)->bool
    pub(crate) file_size: FuncId,               // (path)->fixnum
    pub(crate) file_modified: FuncId,           // (path)->fixnum
    pub(crate) global_get: FuncId,              // (idx)->valor do def global (ADR-0013)
    pub(crate) global_set: FuncId,              // (idx,v)->v (inicializa def global)
    pub(crate) div: FuncId,                     // (a,b)->fixnum|float
    pub(crate) floatp: FuncId,                  // (x)->bool
    pub(crate) double_of: FuncId,               // (x)->float
    pub(crate) stringp: FuncId,                 // (x)->bool
    pub(crate) intp: FuncId,                    // (x)->bool
    pub(crate) keywordp: FuncId,                // (x)->bool
    pub(crate) vectorp: FuncId,                 // (x)->bool
    pub(crate) mapp: FuncId,                    // (x)->bool
    pub(crate) bytesp: FuncId,                  // (x)->bool
    pub(crate) str_split: FuncId,               // (string,char)->vetor de strings
    pub(crate) parse_http_request: FuncId,      // (string)->mapa de requisição (ADR-0013 Gate 4)
    pub(crate) serialize_http_response: FuncId, // (mapa)->string de resposta HTTP
    pub(crate) http_server_open: FuncId,        // (port)->servidor
    pub(crate) http_server_port: FuncId,        // (server)->fixnum
    pub(crate) http_server_accept: FuncId,      // (server)->requisição
    pub(crate) http_server_respond: FuncId,     // (server,resp)->nil
    pub(crate) http_server_close: FuncId,       // (server)->nil
    pub(crate) http_server_stop: FuncId,        // (server)->nil (para o loop)
    pub(crate) float_from_bits: FuncId,         // (raw_i64)->float (literais)
    pub(crate) transient: FuncId,               // (coll)->transient
    pub(crate) persistent_bang: FuncId,         // (t)->coll
    pub(crate) conj_bang: FuncId,               // (t,x)->t
    pub(crate) assoc_bang: FuncId,              // (t,k,v)->t
    pub(crate) dissoc_bang: FuncId,             // (t,k)->t
    pub(crate) make_record: FuncId,             // (type_name,map)->record
    // Protocol and capability dispatch.
    pub(crate) type_key: FuncId,        // (v)->key
    pub(crate) register_method: FuncId, // (mid,key,impl)->void
    pub(crate) lookup_method: FuncId,   // (mid,key)->impl|NIL
    pub(crate) no_method: FuncId,       // (mid)->void
}

pub(crate) fn declare_runtime(m: &mut ObjectModule, ptr: types::Type) -> Runtime {
    let bin = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        s.params.push(AbiParam::new(types::I64));
        s.params.push(AbiParam::new(types::I64));
        s.returns.push(AbiParam::new(types::I64));
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let una = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        s.params.push(AbiParam::new(types::I64));
        s.returns.push(AbiParam::new(types::I64));
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let voidfn = |m: &mut ObjectModule, name: &str, has_arg: bool| {
        let mut s = m.make_signature();
        if has_arg {
            s.params.push(AbiParam::new(types::I64));
        }
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let bin_void = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        s.params.push(AbiParam::new(types::I64));
        s.params.push(AbiParam::new(types::I64));
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let ternary = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        for _ in 0..3 {
            s.params.push(AbiParam::new(types::I64));
        }
        s.returns.push(AbiParam::new(types::I64));
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let ternary_void = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        for _ in 0..3 {
            s.params.push(AbiParam::new(types::I64));
        }
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };
    let quaternary_void = |m: &mut ObjectModule, name: &str| {
        let mut s = m.make_signature();
        for _ in 0..4 {
            s.params.push(AbiParam::new(types::I64));
        }
        m.declare_function(name, Linkage::Import, &s).unwrap()
    };

    let mut str_from_sig = m.make_signature();
    str_from_sig.params.push(AbiParam::new(ptr));
    str_from_sig.params.push(AbiParam::new(types::I64));
    str_from_sig.returns.push(AbiParam::new(types::I64));
    let str_from = m
        .declare_function("cljn_str_from", Linkage::Import, &str_from_sig)
        .unwrap();

    let mut empty_sig = m.make_signature();
    empty_sig.returns.push(AbiParam::new(types::I64));
    let empty = m
        .declare_function("cljn_empty", Linkage::Import, &empty_sig)
        .unwrap();

    let mut truthy_sig = m.make_signature();
    truthy_sig.params.push(AbiParam::new(types::I64));
    truthy_sig.returns.push(AbiParam::new(types::I32));
    let truthy = m
        .declare_function("cljn_truthy", Linkage::Import, &truthy_sig)
        .unwrap();

    Runtime {
        add: bin(m, "cljn_add"),
        sub: bin(m, "cljn_sub"),
        mul: bin(m, "cljn_mul"),
        quot: bin(m, "cljn_quot"),
        mod_: bin(m, "cljn_mod"),
        lt: bin(m, "cljn_lt"),
        le: bin(m, "cljn_le"),
        gt: bin(m, "cljn_gt"),
        ge: bin(m, "cljn_ge"),
        eq: bin(m, "cljn_eq"),
        cons: bin(m, "cljn_cons"),
        str_concat: bin(m, "cljn_str_concat"),
        inc: una(m, "cljn_inc"),
        dec: una(m, "cljn_dec"),
        not_: una(m, "cljn_not"),
        nilp: una(m, "cljn_nilp"),
        emptyp: una(m, "cljn_emptyp"),
        first: una(m, "cljn_first"),
        rest: una(m, "cljn_rest"),
        count: una(m, "cljn_count"),
        to_str: una(m, "cljn_to_str"),
        empty,
        str_from,
        truthy,
        print: voidfn(m, "cljn_print", true),
        print_space: voidfn(m, "cljn_print_space", false),
        print_newline: voidfn(m, "cljn_print_newline", false),
        out_check: voidfn(m, "cljn_out_check", false),
        out_newline: voidfn(m, "cljn_out_newline", false),
        gc_enter: una(m, "cljn_gc_enter"),
        gc_leave: voidfn(m, "cljn_gc_leave", true),
        gc_stack_data: m
            .declare_data("gc_stack", Linkage::Import, true, false)
            .unwrap(),
        const_cache_data: m
            .declare_data("cljn_const_cache", Linkage::Import, true, false)
            .unwrap(),
        const_register: bin_void(m, "cljn_const_register"),
        gc_sp_data: m
            .declare_data("gc_sp", Linkage::Import, true, false)
            .unwrap(),
        make_fn: ternary(m, "cljn_make_fn"),
        set_free: ternary_void(m, "cljn_fn_set_free"),
        fn_free: bin(m, "cljn_fn_free"),
        fn_code: una(m, "cljn_fn_code"),
        check_fn: voidfn(m, "cljn_check_fn", true),
        argv: una(m, "cljn_argv"),
        check_arity: bin_void(m, "cljn_check_arity"),
        collect_rest: ternary(m, "cljn_collect_rest"),
        spread_args: bin(m, "cljn_spread_args"),
        kw: {
            let mut s = m.make_signature();
            s.params.push(AbiParam::new(ptr));
            s.params.push(AbiParam::new(types::I64));
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_kw", Linkage::Import, &s).unwrap()
        },
        vec_empty: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_vec_empty", Linkage::Import, &s)
                .unwrap()
        },
        vec_conj: bin(m, "cljn_vec_conj"),
        set_alloc: una(m, "cljn_set_alloc"),
        set_add: bin_void(m, "cljn_set_add"),
        map_alloc: una(m, "cljn_map_alloc"),
        map_set: quaternary_void(m, "cljn_map_set"),
        p_get: bin(m, "cljn_get"),
        p_nth: bin(m, "cljn_nth"),
        p_nth_or: ternary(m, "cljn_nth_or"),
        p_assoc: ternary(m, "cljn_assoc"),
        p_dissoc: bin(m, "cljn_map_dissoc"),
        p_contains: bin(m, "cljn_contains"),
        p_keys: una(m, "cljn_map_keys"),
        p_vals: una(m, "cljn_map_vals"),
        p_conj: bin(m, "cljn_conj"),
        sorted_map_empty: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_sorted_map_empty", Linkage::Import, &s)
                .unwrap()
        },
        sorted_set_empty: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_sorted_set_empty", Linkage::Import, &s)
                .unwrap()
        },
        sorted_assoc: ternary(m, "cljn_sorted_assoc"),
        compare: bin(m, "cljn_compare"),
        try_: ternary(m, "cljn_try"),
        throw_: una(m, "cljn_throw"),
        multi_register: bin_void(m, "cljn_multi_register"),
        multi_call: ternary(m, "cljn_multi_call"),
        slurp: una(m, "cljn_slurp"),
        spit: bin(m, "cljn_spit"),
        file_exists: una(m, "cljn_file_exists"),
        getenv: una(m, "cljn_getenv"),
        with_out_str: una(m, "cljn_with_out_str"),
        var_get: una(m, "cljn_var_get"),
        with_binding: ternary(m, "cljn_with_binding"),
        read_line: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_read_line", Linkage::Import, &s)
                .unwrap()
        },
        string_reader: una(m, "cljn_string_reader"),
        string_writer: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_string_writer", Linkage::Import, &s)
                .unwrap()
        },
        writer_to_string: una(m, "cljn_writer_to_string"),
        stream_closed: una(m, "cljn_stream_closed"),
        reader_p: una(m, "cljn_reader_p"),
        writer_p: una(m, "cljn_writer_p"),
        read_char_from: una(m, "cljn_read_char_from"),
        read_line_from: una(m, "cljn_read_line_from"),
        unread_char_to: bin(m, "cljn_unread_char_to"),
        write_to: bin(m, "cljn_write_to"),
        flush_writer: una(m, "cljn_flush_writer"),
        closeable_p: una(m, "cljn_closeable_p"),
        char_of: una(m, "cljn_char"),
        int_of: una(m, "cljn_int"),
        charp: una(m, "cljn_charp"),
        path_join: bin(m, "cljn_path_join"),
        file_name: una(m, "cljn_file_name"),
        parent: una(m, "cljn_parent"),
        bytes: una(m, "cljn_bytes"),
        bytes_to_string: una(m, "cljn_bytes_to_string"),
        bget: bin(m, "cljn_bget"),
        bytes_of_vec: una(m, "cljn_bytes_of_vec"),
        bytes_to_vec: una(m, "cljn_bytes_to_vec"),
        valid_utf8: una(m, "cljn_valid_utf8"),
        byte_input_stream: una(m, "cljn_byte_input_stream"),
        byte_output_stream: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_byte_output_stream", Linkage::Import, &s)
                .unwrap()
        },
        read_bytes: bin(m, "cljn_read_bytes"),
        write_bytes: bin(m, "cljn_write_bytes"),
        output_bytes: una(m, "cljn_output_bytes"),
        read_block: bin(m, "cljn_read_block"),
        byte_input_p: una(m, "cljn_byte_input_p"),
        byte_output_p: una(m, "cljn_byte_output_p"),
        seek_file: bin(m, "cljn_seek_file"),
        truncate_file: bin(m, "cljn_truncate_file"),
        position_file: una(m, "cljn_position_file"),
        file_reader_p: una(m, "cljn_file_reader_p"),
        file_writer_p: una(m, "cljn_file_writer_p"),
        create_symlink: bin(m, "cljn_create_symlink"),
        read_link: una(m, "cljn_read_link"),
        native_symlink_p: una(m, "cljn_native_symlink_p"),
        path_absolute: una(m, "cljn_path_absolute"),
        path_normalize: una(m, "cljn_path_normalize"),
        real_path: una(m, "cljn_real_path"),
        process_cwd: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_process_cwd", Linkage::Import, &s)
                .unwrap()
        },
        process_environment: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_process_environment", Linkage::Import, &s)
                .unwrap()
        },
        slurp_bytes: una(m, "cljn_slurp_bytes"),
        spit_bytes: bin(m, "cljn_spit_bytes"),
        read_string: una(m, "cljn_read_string"),
        read_from: una(m, "cljn_read_from"),
        reader_eof: una(m, "cljn_reader_eof"),
        set_args: bin_void(m, "cljn_set_args"),
        writer_open: una(m, "cljn_writer"),
        reader_open: una(m, "cljn_reader"),
        close: una(m, "cljn_close"),
        mkdir: una(m, "cljn_mkdir"),
        mkdirs: una(m, "cljn_mkdirs"),
        list_dir: una(m, "cljn_list_dir"),
        delete_file: una(m, "cljn_delete_file"),
        rename: bin(m, "cljn_rename"),
        directoryp: una(m, "cljn_directoryp"),
        filep: una(m, "cljn_filep"),
        file_size: una(m, "cljn_file_size"),
        file_modified: una(m, "cljn_file_modified"),
        global_get: una(m, "cljn_global_get"),
        global_set: bin(m, "cljn_global_set"),
        div: bin(m, "cljn_div"),
        floatp: una(m, "cljn_floatp"),
        double_of: una(m, "cljn_double"),
        stringp: una(m, "cljn_stringp"),
        intp: una(m, "cljn_intp"),
        keywordp: una(m, "cljn_keywordp"),
        vectorp: una(m, "cljn_vectorp"),
        mapp: una(m, "cljn_mapp"),
        bytesp: una(m, "cljn_bytesp"),
        str_split: bin(m, "cljn_str_split"),
        parse_http_request: una(m, "cljn_parse_http_request"),
        serialize_http_response: una(m, "cljn_serialize_http_response"),
        http_server_open: una(m, "cljn_http_server_open"),
        http_server_port: una(m, "cljn_http_server_port"),
        http_server_accept: una(m, "cljn_http_server_accept"),
        http_server_respond: bin(m, "cljn_http_server_respond"),
        http_server_close: una(m, "cljn_http_server_close"),
        http_server_stop: una(m, "cljn_http_server_stop"),
        float_from_bits: una(m, "cljn_float_from_bits"),
        flush: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_flush", Linkage::Import, &s)
                .unwrap()
        },
        read_char: {
            let mut s = m.make_signature();
            s.returns.push(AbiParam::new(types::I64));
            m.declare_function("cljn_read_char", Linkage::Import, &s)
                .unwrap()
        },
        transient: una(m, "cljn_transient"),
        persistent_bang: una(m, "cljn_persistent_bang"),
        conj_bang: bin(m, "cljn_conj_bang"),
        assoc_bang: ternary(m, "cljn_assoc_bang"),
        dissoc_bang: bin(m, "cljn_dissoc_bang"),
        make_record: bin(m, "cljn_make_record"),
        type_key: una(m, "cljn_type_key"),
        register_method: ternary_void(m, "cljn_register_method"),
        lookup_method: bin(m, "cljn_lookup_method"),
        no_method: voidfn(m, "cljn_no_method", true),
    }
}
