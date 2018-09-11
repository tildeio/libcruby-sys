#include "ruby/ruby.h"
#include "stdint.h"

VALUE RS_Qtrue = Qtrue;
VALUE RS_Qfalse = Qfalse;
VALUE RS_Qnil = Qnil;

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
