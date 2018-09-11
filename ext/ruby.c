#include "ruby/ruby.h"
#include "stdint.h"

typedef void (*RS_RUBY_DATA_FUNC)(void*);

VALUE RS_Qtrue = Qtrue;
VALUE RS_Qfalse = Qfalse;
VALUE RS_Qnil = Qnil;


/** Macro Wrappers **/

long RS_RSTRING_LEN(VALUE str) { return RSTRING_LEN(str); }

// This gives a direct reference to a `char *` which could be mutated,
// but this seems quite risky, so we'll call it a `const`.
const char* RS_RSTRING_PTR(VALUE str) { return RSTRING_PTR(str); }

long RS_RARRAY_LEN(VALUE a) { return RARRAY_LEN(a); }

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
