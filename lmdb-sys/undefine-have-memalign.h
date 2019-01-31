/* Undefine HAVE_MEMALIGN, as mdb.c expects it to be undefined and calls
 * memalign() without including alloc.h if it's defined in the environment,
 * even if HAVE_POSIX_MEMALIGN is also defined; and unfortunately some build
 * environments define HAVE_MEMALIGN even if they HAVE_POSIX_MEMALIGN
 * (per https://bugzilla.mozilla.org/show_bug.cgi?id=1512541), which triggers
 * an implicit function declaration and linker bustage on some systems.
 *
 * We undefine HAVE_MEMALIGN by including this header file rather than setting
 * a compiler flag (-UHAVE_MEMALIGN) because the aforementioned build
 * environment defines it via a header file (mozilla-config.h), which would
 * override the compiler flag.
 */
#ifndef UNDEFINE_HAVE_MEMALIGN_H
#define UNDEFINE_HAVE_MEMALIGN_H
#undef HAVE_MEMALIGN
#endif /* UNDEFINE_HAVE_MEMALIGN_H */
