#include "ruby/ruby.h"
#include "stdint.h"

VALUE RS_Qtrue = Qtrue;
VALUE RS_Qfalse = Qfalse;
VALUE RS_Qnil = Qnil;

// begin ruby_value_type
int RS_T_NONE = RUBY_T_NONE;

int RS_T_OBJECT = RUBY_T_OBJECT;
int RS_T_CLASS = RUBY_T_CLASS;
int RS_T_MODULE = RUBY_T_MODULE;
int RS_T_FLOAT = RUBY_T_FLOAT;
int RS_T_STRING = RUBY_T_STRING;
int RS_T_REGEXP = RUBY_T_REGEXP;
int RS_T_ARRAY = RUBY_T_ARRAY;
int RS_T_HASH = RUBY_T_HASH;
int RS_T_STRUCT = RUBY_T_STRUCT;
int RS_T_BIGNUM = RUBY_T_BIGNUM;
int RS_T_FILE = RUBY_T_FILE;
int RS_T_DATA = RUBY_T_DATA;
int RS_T_MATCH = RUBY_T_MATCH;
int RS_T_COMPLEX = RUBY_T_COMPLEX;
int RS_T_RATIONAL = RUBY_T_RATIONAL;

int RS_T_NIL = RUBY_T_NIL;
int RS_T_TRUE = RUBY_T_TRUE;
int RS_T_FALSE = RUBY_T_FALSE;
int RS_T_SYMBOL = RUBY_T_SYMBOL;
int RS_T_FIXNUM = RUBY_T_FIXNUM;
int RS_T_UNDEF = RUBY_T_UNDEF;

int RS_T_IMEMO = RUBY_T_IMEMO;
int RS_T_NODE = RUBY_T_NODE;
int RS_T_ICLASS = RUBY_T_ICLASS;
int RS_T_ZOMBIE = RUBY_T_ZOMBIE;
// end ruby_value_type

int RS_RB_TYPE_P(VALUE obj, int type) { return RB_TYPE_P(obj, type); }

VALUE RS_CLASS_OF(VALUE v) { return CLASS_OF(v); }

long RS_RSTRING_LEN(VALUE str) { return RSTRING_LEN(str); }

// This gives a direct reference to a `char *` which could be mutated,
// but this seems quite risky, so we'll call it a `const`.
const char* RS_RSTRING_PTR(VALUE str) { return RSTRING_PTR(str); }

long RS_RARRAY_LEN(VALUE a) { return RARRAY_LEN(a); }
