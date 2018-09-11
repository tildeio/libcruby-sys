#include "ruby/ruby.h"
#include "stdint.h"

VALUE RS_Qtrue = Qtrue;
VALUE RS_Qfalse = Qfalse;
VALUE RS_Qnil = Qnil;

long RS_RSTRING_LEN(VALUE str) { return RSTRING_LEN(str); }

// This gives a direct reference to a `char *` which could be mutated,
// but this seems quite risky, so we'll call it a `const`.
const char* RS_RSTRING_PTR(VALUE str) { return RSTRING_PTR(str); }

long RS_RARRAY_LEN(VALUE a) { return RARRAY_LEN(a); }

size_t RS_RHASH_SIZE(VALUE h) { return RHASH_SIZE(h); }
