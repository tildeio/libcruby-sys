#include "ruby/ruby.h"
#include "stdint.h"

typedef void (*RS_RUBY_DATA_FUNC)(void*);

const char* RS_PRIsVALUE = PRIsVALUE;

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

int RS_T_MASK = RUBY_T_MASK;
// end ruby_value_type

/** Macro Wrappers **/

int RS_RB_TYPE_P(VALUE obj, int type) { return RB_TYPE_P(obj, type); }

int RS_NUM2INT(VALUE num) { return NUM2INT(num); }
VALUE RS_INT2NUM(int i) { return INT2NUM(i); }

unsigned int RS_NUM2UINT(VALUE num) { return NUM2UINT(num); }
VALUE RS_UINT2NUM(unsigned int ui) { return UINT2NUM(ui); }

long RS_NUM2LONG(VALUE num) { return NUM2LONG(num); }
VALUE RS_LONG2NUM(long l) { return LONG2NUM(l); }

unsigned long RS_NUM2ULONG(VALUE num) { return NUM2ULONG(num); }
VALUE RS_ULONG2NUM(unsigned long ul) { return ULONG2NUM(ul); }

// Do we support any platforms that wouldn't have `long long`?
// If so, these won't be present.
long long RS_NUM2LL(VALUE num) { return NUM2LL(num); }
VALUE RS_LL2NUM(long long ll) { return LL2NUM(ll); }

unsigned long long RS_NUM2ULL(VALUE num) { return NUM2ULL(num); }
VALUE RS_ULL2NUM(unsigned long long ull) { return ULL2NUM(ull); }

double RS_NUM2DBL(VALUE num) { return NUM2DBL(num); }
VALUE RS_DBL2NUM(double dbl) { return DBL2NUM(dbl); }

long RS_RSTRING_LEN(VALUE str) { return RSTRING_LEN(str); }

// This gives a direct reference to a `char *` which could be mutated,
// but this seems quite risky, so we'll call it a `const`.
const char* RS_RSTRING_PTR(VALUE str) { return RSTRING_PTR(str); }

long RS_RARRAY_LEN(VALUE a) { return RARRAY_LEN(a); }

// If the arch supports it, it's a ull, otherwise just a ulong.
// Should be ok to do ull here.
unsigned long long RS_RHASH_SIZE(VALUE h) { return RHASH_SIZE(h); }

VALUE RS_Data_Wrap_Struct(VALUE klass, RS_RUBY_DATA_FUNC mark, RS_RUBY_DATA_FUNC free, void* sval) {
  return Data_Wrap_Struct(klass, mark, free, sval);
}


/** Custom Methods **/

// The standard C API recommendation is to use `Data_Get_Struct` and mutate the struct directly.
// However, that doesn't work well with Rust so we implement two new methods here to make it possible.
void* RS_Data_Get_Struct_Value(VALUE obj) {
  void* sval;
  Data_Get_Struct(obj, void*, sval);
  return sval;
}

void RS_Data_Set_Struct_Value(VALUE obj, void* sval) {
  DATA_PTR(obj) = sval;
}
