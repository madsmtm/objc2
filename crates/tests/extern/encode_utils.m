#include <objc/objc.h>
#include <stdint.h>
#include <stddef.h>
#include <uuid/uuid.h>
// For NSInteger / NSUInteger. Linking is not required.
#include <Foundation/NSObject.h>

#define ENCODING_INNER(name, type) char* ENCODING_ ## name = @encode(type);

#define ENCODING_NO_ATOMIC(name, type) \
    ENCODING_INNER(name, type); \
    ENCODING_INNER(name ## _POINTER, type*); \
    ENCODING_INNER(name ## _POINTER_POINTER, type**); \
    ENCODING_INNER(name ## _POINTER_POINTER_POINTER, type***); \
    ENCODING_INNER(name ## _POINTER_ATOMIC, _Atomic (type*));

#define ENCODING(name, type) \
    ENCODING_NO_ATOMIC(name, type); \
    ENCODING_INNER(name ## _ATOMIC_POINTER, _Atomic type*); \
    ENCODING_INNER(name ## _ATOMIC, _Atomic type);

// C types

ENCODING(C99_BOOL, _Bool);
ENCODING(CHAR, char);
ENCODING(SIGNED_CHAR, signed char);
ENCODING(UNSIGNED_CHAR, unsigned char);
ENCODING(SHORT, short);
ENCODING(UNSIGNED_SHORT, unsigned short);
ENCODING(INT, int);
ENCODING(UNSIGNED_INT, unsigned int);
ENCODING(LONG, long);
ENCODING(UNSIGNED_LONG, unsigned long);
ENCODING(LONG_LONG, long long);
ENCODING(UNSIGNED_LONG_LONG, unsigned long long);
ENCODING(FLOAT, float);
ENCODING(DOUBLE, double);
ENCODING(LONG_DOUBLE, long double);

ENCODING(FLOAT_COMPLEX, float _Complex);
ENCODING(DOUBLE_COMPLEX, double _Complex);
ENCODING(LONG_DOUBLE_COMPLEX, long double _Complex);
// TODO: Enable these:
// ENCODING(FLOAT_IMAGINARY, float _Imaginary);
// ENCODING(DOUBLE_IMAGINARY, double _Imaginary);
// ENCODING(LONG_DOUBLE_IMAGINARY, long double _Imaginary);

ENCODING_NO_ATOMIC(VOID, void);
ENCODING_INNER(VOID_POINTER_CONST, const void*);

// Struct

struct empty {};
ENCODING(STRUCT_EMPTY, struct empty);

struct one_item {
    void* a;
};
ENCODING(STRUCT_ONE_ITEM, struct one_item);

struct nested {
    struct one_item a;
    struct one_item* b;
};
ENCODING(STRUCT_NESTED, struct nested);

struct two_items {
    float a;
    int b;
};
ENCODING(STRUCT_TWO_ITEMS, struct two_items);

struct with_arrays {
    int a[1];
    int* b[2];
    int (*c)[3];
};
ENCODING(STRUCT_WITH_ARRAYS, struct with_arrays);

struct with_block {
    void (^a)(void);
    id b;
    void (*c)(void);
};
ENCODING_NO_ATOMIC(STRUCT_WITH_BLOCK, struct with_block); \

struct with_atomic_inner {
    _Atomic int a;
    _Atomic int* b;
    _Atomic (int*) c;
};
struct with_atomic {
    _Atomic int a;
    _Atomic const int* b;
    _Atomic (int*) c;
    struct with_atomic_inner d;
    struct with_atomic_inner* e;
    _Atomic struct with_atomic_inner* f;
};
ENCODING(STRUCT_WITH_ATOMIC, struct with_atomic);

// Bit field

struct bitfield {
    unsigned int a: 1;
    unsigned int b: 30;
};
ENCODING(BITFIELD, struct bitfield);

// Union

union union_ {
    float a;
    int b;
};
ENCODING(UNION, union union_);

// Array
// Using typedefs because the type of pointers to arrays are hard to name.
// Also, atomic arrays does not exist

typedef int arr[10];
ENCODING_NO_ATOMIC(ARRAY_INT, arr);

typedef int* arr_ptr[10];
ENCODING_NO_ATOMIC(ARRAY_POINTER, arr_ptr);

typedef int arr_nested[10][20];
ENCODING_NO_ATOMIC(ARRAY_NESTED, arr_nested);

typedef struct two_items arr_struct[0];
ENCODING_NO_ATOMIC(ARRAY_STRUCT, arr_struct);

// Objective-C

ENCODING(OBJC_BOOL, BOOL);
ENCODING_INNER(ID, id);
ENCODING_INNER(ID_POINTER, const id*);
ENCODING_INNER(ID_POINTER_POINTER, const id**);
ENCODING_INNER(ID_POINTER_POINTER_POINTER, const id***);
ENCODING_INNER(ID_ATOMIC, _Atomic id);
ENCODING_INNER(ID_ATOMIC_POINTER, _Atomic const id*);
ENCODING_INNER(ID_POINTER_ATOMIC, _Atomic (const id*));
ENCODING(CLASS, Class);
ENCODING(SEL, SEL);
ENCODING(NS_INTEGER, NSInteger);
ENCODING(NS_UINTEGER, NSUInteger);

// stdint.h

ENCODING(INT8, int8_t);
ENCODING(INT16, int16_t);
ENCODING(INT32, int32_t);
ENCODING(INT64, int64_t);
ENCODING(INTPTR, intptr_t);
ENCODING(UINT8, uint8_t);
ENCODING(UINT16, uint16_t);
ENCODING(UINT32, uint32_t);
ENCODING(UINT64, uint64_t);
ENCODING(UINTPTR, uintptr_t);

// stddef.h

ENCODING(SIZE_T, size_t);
ENCODING(PTRDIFF_T, ptrdiff_t);

// uuid.h

ENCODING_NO_ATOMIC(UUID_T, uuid_t);

// Possible extras

#if __has_builtin(__int128_t)
ENCODING(SIGNED_INT_128, __int128_t);
ENCODING(UNSIGNED_INT_128, __uint128_t);
#endif
