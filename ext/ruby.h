#include <ruby/ruby.h>
#include <ruby/intern.h>

#if defined _WIN32 || defined __CYGWIN__
  #ifdef BUILDING_DLL
    #define LIBC_RUBY_EXTERN __declspec(dllexport)
  #else
    #define LIBC_RUBY_EXTERN __declspec(dllimport)
  #endif
#else
  #define LIBC_RUBY_EXTERN extern
#endif

LIBC_RUBY_EXTERN VALUE RS_Qtrue;
LIBC_RUBY_EXTERN VALUE RS_Qfalse;
LIBC_RUBY_EXTERN VALUE RS_Qnil;