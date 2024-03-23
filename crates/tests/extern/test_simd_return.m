#include <Foundation/NSObject.h>
#include <simd/simd.h>

@interface TestSimdReturn: NSObject
@end

#define METHOD(type) + (simd_ ## type) type
#define METHOD_POW2(type, from) + (simd_ ## type) type { return simd_make_ ## type([self from], [self from]); }

@implementation TestSimdReturn
METHOD(float1) {
    return 42;
}
METHOD_POW2(float2, float1)
METHOD(float3) {
    return simd_make_float3([self float1], [self float1], [self float1]);
}
METHOD_POW2(float4, float2)
METHOD_POW2(float8, float4)
METHOD_POW2(float16, float8)

METHOD(char1) {
    return 42;
}
METHOD_POW2(char2, char1)
METHOD(char3) {
    return simd_make_char3([self char1], [self char1], [self char1]);
}
METHOD_POW2(char4, char2)
METHOD_POW2(char8, char4)
METHOD_POW2(char16, char8)
METHOD_POW2(char32, char16)
METHOD_POW2(char64, char32)

METHOD(quatf) {
    return simd_quaternion([self float4]);
}

METHOD(float2x2) {
    return simd_matrix([self float2], [self float2]);
}

METHOD(float2x4) {
    return simd_matrix([self float4], [self float4]);
}

METHOD(float4x4) {
    return simd_matrix([self float4], [self float4], [self float4], [self float4]);
}
@end
