data! {
    class NSArray {
        unsafe -count;
    }

    class NSMutableArray: Owned {
        unsafe mut -removeAllObjects;
        mut -addObject;
        mut -insertObject_atIndex;
        mut -replaceObjectAtIndex_withObject;
        mut -removeObjectAtIndex;
        mut -removeLastObject;
        mut -sortUsingFunction_context;
    }

    class NSString {
        unsafe -init;
        unsafe -compare;
        unsafe -hasPrefix;
        unsafe -hasSuffix;
        // The other string is non-null, and won't be retained
        unsafe -stringByAppendingString;
        unsafe -stringByAppendingPathComponent;
        // Assuming `NSStringEncoding` can be made safe
        unsafe -lengthOfBytesUsingEncoding;
        unsafe -length;
        // Safe to call, but the returned pointer may not be safe to use
        unsafe -UTF8String;
        unsafe -initWithString;
        unsafe +stringWithString;
    }

    class NSMutableString: Owned {
        unsafe -initWithCapacity;
        unsafe +stringWithCapacity;
        unsafe -initWithString;
        unsafe +stringWithString;
        unsafe mut -appendString;
        unsafe mut -setString;
    }

    class NSAttributedString {
        unsafe -initWithString;
        unsafe -initWithAttributedString;
        unsafe -string;
        unsafe -length;
    }

    class NSMutableAttributedString: Owned {
        unsafe -initWithString;
        unsafe -initWithAttributedString;
        unsafe mut -setAttributedString;
    }

    class NSBundle {
        unsafe +mainBundle;
        unsafe -infoDictionary;
    }

    class NSData {
        unsafe -initWithData;
        unsafe +dataWithData;
        unsafe -length;
        unsafe -bytes;
    }

    class NSMutableData: Owned {
        unsafe +dataWithData;
        unsafe -initWithCapacity;
        unsafe +dataWithCapacity;
        unsafe mut -setLength;
        unsafe mut -mutableBytes;
    }

    class NSDictionary {
        unsafe -count;
        unsafe -objectForKey;
        unsafe -allKeys;
        unsafe -allValues;
    }

    class NSMutableDictionary: Owned {
        unsafe mut -setDictionary;
        unsafe mut -removeObjectForKey;
        unsafe mut -removeAllObjects;
    }

    class NSError {
        unsafe -domain;
        unsafe -code;
        unsafe -userInfo;
        unsafe -localizedDescription;
    }

    class NSException {
        unsafe -name;
        unsafe -reason;
        unsafe -userInfo;
    }

    class NSLock {
        unsafe -name;
        unsafe -setName;
    }

    class NSValue {
        unsafe -objCType;
        unsafe -isEqualToValue;
    }

    class NSUUID {
        unsafe +UUID;
        unsafe -init;
        unsafe -initWithUUIDString;
        unsafe -UUIDString;
    }

    class NSThread {
        unsafe +currentThread;
        unsafe +mainThread;
        unsafe -name;
        unsafe +isMultiThreaded;
        unsafe -isMainThread;
        unsafe +isMainThread;
    }

    class NSProcessInfo {
        unsafe +processInfo;
        unsafe -processName;
        unsafe -operatingSystemVersion;
    }

    class NSSet {
        unsafe -count;
    }

    class NSMutableSet: Owned {
        unsafe mut -removeAllObjects;
        mut -addObject;
    }

    class NSMutableCharacterSet: Owned {}

    class NSMutableOrderedSet: Owned {}

    class NSMutableIndexSet: Owned {}

    class NSNumber {
        unsafe -initWithChar;
        unsafe -initWithUnsignedChar;
        unsafe -initWithShort;
        unsafe -initWithUnsignedShort;
        unsafe -initWithInt;
        unsafe -initWithUnsignedInt;
        unsafe -initWithLong;
        unsafe -initWithUnsignedLong;
        unsafe -initWithLongLong;
        unsafe -initWithUnsignedLongLong;
        unsafe -initWithFloat;
        unsafe -initWithDouble;
        unsafe -initWithBool;
        unsafe -initWithInteger;
        unsafe -initWithUnsignedInteger;
        unsafe +numberWithChar;
        unsafe +numberWithUnsignedChar;
        unsafe +numberWithShort;
        unsafe +numberWithUnsignedShort;
        unsafe +numberWithInt;
        unsafe +numberWithUnsignedInt;
        unsafe +numberWithLong;
        unsafe +numberWithUnsignedLong;
        unsafe +numberWithLongLong;
        unsafe +numberWithUnsignedLongLong;
        unsafe +numberWithFloat;
        unsafe +numberWithDouble;
        unsafe +numberWithBool;
        unsafe +numberWithInteger;
        unsafe +numberWithUnsignedInteger;
        unsafe -compare;
        unsafe -isEqualToNumber;
        unsafe -charValue;
        unsafe -unsignedCharValue;
        unsafe -shortValue;
        unsafe -unsignedShortValue;
        unsafe -intValue;
        unsafe -unsignedIntValue;
        unsafe -longValue;
        unsafe -unsignedLongValue;
        unsafe -longLongValue;
        unsafe -unsignedLongLongValue;
        unsafe -floatValue;
        unsafe -doubleValue;
        unsafe -boolValue;
        unsafe -integerValue;
        unsafe -unsignedIntegerValue;
        unsafe -stringValue;
    }

    class NSMutableURLRequest: Owned {}
}
