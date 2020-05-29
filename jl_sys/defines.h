#define container_of(ptr, type, member) \
    ((type *) ((char *)(ptr) - offsetof(type, member)))
#define jl_astaggedvalue(v)                                             \
    ((jl_taggedvalue_t*)((char*)(v) - sizeof(jl_taggedvalue_t)))
#define jl_valueof(v)                                           \
    ((jl_value_t*)((char*)(v) + sizeof(jl_taggedvalue_t)))
#define jl_typeof(v)                                                    \
    ((jl_value_t*)(jl_astaggedvalue(v)->header & ~(uintptr_t)15))
#define jl_typeis(v,t) (jl_typeof(v)==(jl_value_t*)(t))
#define jl_tuple_type jl_anytuple_type
#define jl_pgcstack (jl_get_ptls_states()->pgcstack)
#define jl_svec_len(t)              (((jl_svec_t*)(t))->length)
#define jl_svec_set_len_unsafe(t,n) (((jl_svec_t*)(t))->length=(n))
#define jl_svec_data(t) ((jl_value_t**)((char*)(t) + sizeof(jl_svec_t)))
#define jl_array_len(a)   (((jl_array_t*)(a))->length)
#define jl_array_data(a)  ((void*)((jl_array_t*)(a))->data)
#define jl_array_dim(a,i) ((&((jl_array_t*)(a))->nrows)[i])
#define jl_array_dim0(a)  (((jl_array_t*)(a))->nrows)
#define jl_array_nrows(a) (((jl_array_t*)(a))->nrows)
#define jl_array_ndims(a) ((int32_t)(((jl_array_t*)a)->flags.ndims))
#define jl_array_data_owner_offset(ndims) (offsetof(jl_array_t,ncols) + sizeof(size_t)*(1+jl_array_ndimwords(ndims))) // in bytes
#define jl_array_data_owner(a) (*((jl_value_t**)((char*)a + jl_array_data_owner_offset(jl_array_ndims(a)))))
#define jl_array_ptr_data(a)  ((jl_value_t**)((jl_array_t*)(a))->data)
#define jl_exprarg(e,n) jl_array_ptr_ref(((jl_expr_t*)(e))->args, n)
#define jl_exprargset(e, n, v) jl_array_ptr_set(((jl_expr_t*)(e))->args, n, v)
#define jl_expr_nargs(e) jl_array_len(((jl_expr_t*)(e))->args)
#define jl_fieldref(s,i) jl_get_nth_field(((jl_value_t*)(s)),i)
#define jl_fieldref_noalloc(s,i) jl_get_nth_field_noalloc(((jl_value_t*)(s)),i)
#define jl_nfields(v)    jl_datatype_nfields(jl_typeof(v))
#define jl_linenode_line(x) (((intptr_t*)(x))[0])
#define jl_linenode_file(x) (((jl_value_t**)(x))[1])
#define jl_slot_number(x) (((intptr_t*)(x))[0])
#define jl_typedslot_get_type(x) (((jl_value_t**)(x))[1])
#define jl_gotonode_label(x) (((intptr_t*)(x))[0])
#define jl_globalref_mod(s) (*(jl_module_t**)(s))
#define jl_globalref_name(s) (((jl_sym_t**)(s))[1])
#define jl_quotenode_value(x) (((jl_value_t**)x)[0])
#define jl_nparams(t)  jl_svec_len(((jl_datatype_t*)(t))->parameters)
#define jl_tparam0(t)  jl_svecref(((jl_datatype_t*)(t))->parameters, 0)
#define jl_tparam1(t)  jl_svecref(((jl_datatype_t*)(t))->parameters, 1)
#define jl_tparam(t,i) jl_svecref(((jl_datatype_t*)(t))->parameters, i)
#define jl_data_ptr(v)  ((jl_value_t**)v)
#define jl_string_data(s) ((char*)s + sizeof(void*))
#define jl_string_len(s)  (*(size_t*)s)
#define jl_gf_mtable(f) (((jl_datatype_t*)jl_typeof(f))->name->mt)
#define jl_gf_name(f)   (jl_gf_mtable(f)->name)
#define jl_get_fieldtypes(st) ((st)->types ? (st)->types : jl_compute_fieldtypes((st)))
#define jl_datatype_size(t)    (((jl_datatype_t*)t)->size)
#define jl_datatype_align(t)   (((jl_datatype_t*)t)->layout->alignment)
#define jl_datatype_nbits(t)   ((((jl_datatype_t*)t)->size)*8)
#define jl_datatype_nfields(t) (((jl_datatype_t*)(t))->layout->nfields)
#define jl_datatype_isinlinealloc(t) (((jl_datatype_t *)(t))->isinlinealloc)
#define jl_symbol_name(s) jl_symbol_name_(s)
#define jl_dt_layout_fields(d) ((const char*)(d) + sizeof(jl_datatype_layout_t))
#define jl_is_nothing(v)     (((jl_value_t*)(v)) == ((jl_value_t*)jl_nothing))
#define jl_is_tuple(v)       (((jl_datatype_t*)jl_typeof(v))->name == jl_tuple_typename)
#define jl_is_namedtuple(v)  (((jl_datatype_t*)jl_typeof(v))->name == jl_namedtuple_typename)
#define jl_is_svec(v)        jl_typeis(v,jl_simplevector_type)
#define jl_is_simplevector(v) jl_is_svec(v)
#define jl_is_datatype(v)    jl_typeis(v,jl_datatype_type)
#define jl_is_mutable(t)     (((jl_datatype_t*)t)->mutabl)
#define jl_is_mutable_datatype(t) (jl_is_datatype(t) && (((jl_datatype_t*)t)->mutabl))
#define jl_is_immutable(t)   (!((jl_datatype_t*)t)->mutabl)
#define jl_is_immutable_datatype(t) (jl_is_datatype(t) && (!((jl_datatype_t*)t)->mutabl))
#define jl_is_uniontype(v)   jl_typeis(v,jl_uniontype_type)
#define jl_is_typevar(v)     jl_typeis(v,jl_tvar_type)
#define jl_is_unionall(v)    jl_typeis(v,jl_unionall_type)
#define jl_is_typename(v)    jl_typeis(v,jl_typename_type)
#define jl_is_int8(v)        jl_typeis(v,jl_int8_type)
#define jl_is_int16(v)       jl_typeis(v,jl_int16_type)
#define jl_is_int32(v)       jl_typeis(v,jl_int32_type)
#define jl_is_int64(v)       jl_typeis(v,jl_int64_type)
#define jl_is_uint8(v)       jl_typeis(v,jl_uint8_type)
#define jl_is_uint16(v)      jl_typeis(v,jl_uint16_type)
#define jl_is_uint32(v)      jl_typeis(v,jl_uint32_type)
#define jl_is_uint64(v)      jl_typeis(v,jl_uint64_type)
#define jl_is_bool(v)        jl_typeis(v,jl_bool_type)
#define jl_is_symbol(v)      jl_typeis(v,jl_symbol_type)
#define jl_is_ssavalue(v)    jl_typeis(v,jl_ssavalue_type)
#define jl_is_slot(v)        (jl_typeis(v,jl_slotnumber_type) || jl_typeis(v,jl_typedslot_type))
#define jl_is_expr(v)        jl_typeis(v,jl_expr_type)
#define jl_is_globalref(v)   jl_typeis(v,jl_globalref_type)
#define jl_is_gotonode(v)    jl_typeis(v,jl_gotonode_type)
#define jl_is_pinode(v)      jl_typeis(v,jl_pinode_type)
#define jl_is_phinode(v)     jl_typeis(v,jl_phinode_type)
#define jl_is_phicnode(v)    jl_typeis(v,jl_phicnode_type)
#define jl_is_upsilonnode(v) jl_typeis(v,jl_upsilonnode_type)
#define jl_is_quotenode(v)   jl_typeis(v,jl_quotenode_type)
#define jl_is_newvarnode(v)  jl_typeis(v,jl_newvarnode_type)
#define jl_is_linenode(v)    jl_typeis(v,jl_linenumbernode_type)
#define jl_is_method_instance(v) jl_typeis(v,jl_method_instance_type)
#define jl_is_code_instance(v) jl_typeis(v,jl_code_instance_type)
#define jl_is_code_info(v)   jl_typeis(v,jl_code_info_type)
#define jl_is_method(v)      jl_typeis(v,jl_method_type)
#define jl_is_module(v)      jl_typeis(v,jl_module_type)
#define jl_is_mtable(v)      jl_typeis(v,jl_methtable_type)
#define jl_is_task(v)        jl_typeis(v,jl_task_type)
#define jl_is_string(v)      jl_typeis(v,jl_string_type)
#define jl_is_cpointer(v)    jl_is_cpointer_type(jl_typeof(v))
#define jl_is_pointer(v)     jl_is_cpointer_type(jl_typeof(v))
#define jl_is_intrinsic(v)   jl_typeis(v,jl_intrinsic_type)
#define jl_array_isbitsunion(a) (!(((jl_array_t*)(a))->flags.ptrarray) && jl_is_uniontype(jl_tparam0(jl_typeof(a))))
#define jl_box_long(x)   jl_box_int64(x)
#define jl_box_ulong(x)  jl_box_uint64(x)
#define jl_unbox_long(x) jl_unbox_int64(x)
#define jl_unbox_ulong(x) jl_unbox_uint64(x)
#define jl_is_long(x)    jl_is_int64(x)
#define jl_long_type     jl_int64_type
#define jl_ulong_type    jl_uint64_type
#define julia_init julia_init__threading
#define jl_init jl_init__threading
#define jl_init_with_image jl_init_with_image__threading
#define jl_setjmp_f    __sigsetjmp
#define jl_setjmp_name "__sigsetjmp"
#define jl_setjmp(a,b) sigsetjmp(a,b)
#define jl_longjmp(a,b) siglongjmp(a,b)
#define jl_current_task (jl_get_ptls_states()->current_task)
#define jl_root_task (jl_get_ptls_states()->root_task)