extern crate libc;


pub type jvoid = ::libc::c_void;
pub type jboolean = ::libc::c_uchar;
pub type jbyte = ::libc::c_char;
pub type jchar = ::libc::c_ushort;
pub type jshort = ::libc::c_short;
pub type jint = ::libc::c_int;
pub type jlong = i64;
pub type jfloat = ::libc::c_float;
pub type jdouble = ::libc::c_double;
pub type jsize = jint;

struct jobject_impl;
pub type jobject = *jobject_impl;
pub type jclass = jobject;
pub type jthrowable = jobject;
pub type jstring = jobject;
pub type jarray = jobject;
pub type jbooleanArray = jobject;
pub type jbyteArray = jobject;
pub type jcharArray = jobject;
pub type jshortArray = jobject;
pub type jintArray = jobject;
pub type jlongArray = jobject;
pub type jfloatArray = jobject;
pub type jdoubleArray = jobject;
pub type jobjectArray = jobject;

pub type jweak = jobject;


// TODO: deal with repr
pub enum jvalue {
	Jz(jboolean),
	Jb(jbyte),
	Jc(jchar),
	Js(jshort),
	Ji(jint),
	Jj(jlong),
	Jf(jfloat),
	Jd(jdouble),
	Jl(jobject)
}


struct jfieldID_impl;
pub type jfieldID = *jfieldID_impl;

struct jmethodID_impl;
pub type jmethodID = *jmethodID_impl;

pub static JNI_FALSE: jboolean = 0;
pub static JNI_TRUE: jboolean = 1;

#[deriving(Show, Clone, FromPrimitive)]
#[repr(int)]
pub enum JniVersion {
	JNI_VERSION_1_1 = 0x00010001,
	JNI_VERSION_1_2 = 0x00010002,
	JNI_VERSION_1_4 = 0x00010004,
	JNI_VERSION_1_6 = 0x00010006
}

#[deriving(Show, Clone, PartialEq, Eq, FromPrimitive)]
#[repr(int)]
pub enum JniError {
	JNI_OK =            0,              /* success */
	JNI_ERR =          -1,              /* unknown error */
	JNI_EDETACHED =    -2,              /* thread detached from the VM */
	JNI_EVERSION =     -3,              /* JNI version error */
	JNI_ENOMEM =       -4,              /* not enough memory */
	JNI_EEXIST =       -5,              /* VM already created */
	JNI_EINVAL =       -6               /* invalid arguments */
}

#[deriving(Show, Clone, FromPrimitive)]
#[repr(int)]
pub enum JniReleaseArrayElementsMode {
	JNI_ZERO = 0,
	JNI_COMMIT = 1,
	JNI_ABORT = 2
}

pub struct JNIInvokeInterface {
	#[allow(dead_code)]
	reserved0: *jvoid,
	#[allow(dead_code)]
	reserved1: *jvoid,
	#[allow(dead_code)]
	reserved2: *jvoid,

	pub DestroyJavaVM: extern "C" fn(vm: *JavaVMImpl) -> JniError,
	pub AttachCurrentThread: extern "C" fn(vm: *JavaVMImpl, penv: &mut *JNIEnvImpl, args: *JavaVMAttachArgsImpl) -> JniError,
	pub DetachCurrentThread: extern "C" fn(vm: *JavaVMImpl) -> JniError,
	pub GetEnv: extern "C" fn(vm: *JavaVMImpl, penv: &mut *JNIEnvImpl, version: JniVersion) -> JniError,
	pub AttachCurrentThreadAsDaemon: extern "C" fn(vm: *JavaVMImpl, penv: &mut *JNIEnvImpl, args: *JavaVMAttachArgsImpl) -> JniError
}

pub type JavaVMImpl = *JNIInvokeInterface;

pub struct JNINativeInterface {
	#[allow(dead_code)]
	reserved0: *jvoid,
	#[allow(dead_code)]
	reserved1: *jvoid,
	#[allow(dead_code)]
	reserved2: *jvoid,
	#[allow(dead_code)]
	reserved3: *jvoid,

	pub GetVersion:          extern "C" fn(env: *JNIEnvImpl) -> JniVersion,

	pub DefineClass:         extern "C" fn(env: *JNIEnvImpl, name: *::libc::c_char, loader: jobject, buf: *jbyte, len: jsize) -> jclass,

	pub FindClass:           extern "C" fn(env: *JNIEnvImpl, name: *::libc::c_char) -> jclass,

	pub FromReflectedMethod: extern "C" fn(env: *JNIEnvImpl, method: jobject) -> jmethodID,

	pub FromReflectedField:  extern "C" fn(env: *JNIEnvImpl, field: jobject) -> jmethodID,

	pub ToReflectedMethod:   extern "C" fn(env: *JNIEnvImpl, cls: jclass, methodID: jmethodID, isStatic: jboolean) -> jmethodID,

	pub GetSuperclass:       extern "C" fn(env: *JNIEnvImpl, sub: jclass) -> jclass,

	pub IsAssignableFrom:    extern "C" fn(env: *JNIEnvImpl, sub: jclass, sup: jclass) -> jboolean,

	pub ToReflectedField:    extern "C" fn(env: *JNIEnvImpl, cls: jclass, fieldID: jfieldID, isStatic: jboolean) -> jobject,

	pub Throw:               extern "C" fn(env: *JNIEnvImpl, obj: jthrowable) -> JniError,
	pub ThrowNew:            extern "C" fn(env: *JNIEnvImpl, clazz: jclass, msg: *::libc::c_char) -> JniError,
	pub ExceptionOccurred:   extern "C" fn(env: *JNIEnvImpl) -> jthrowable,
	pub ExceptionDescribe:   extern "C" fn(env: *JNIEnvImpl),
	pub ExceptionClear:      extern "C" fn(env: *JNIEnvImpl),
	pub FatalError:          extern "C" fn(env: *JNIEnvImpl, msg: *::libc::c_char),

	pub PushLocalFrame:      extern "C" fn(env: *JNIEnvImpl, capacity: jint) -> JniError,
	pub PopLocalFrame:       extern "C" fn(env: *JNIEnvImpl, result: jobject) -> jobject,

	pub NewGlobalRef:        extern "C" fn(env: *JNIEnvImpl, lobj: jobject) -> jobject,
	pub DeleteGlobalRef:     extern "C" fn(env: *JNIEnvImpl, gref: jobject),
	pub DeleteLocalRef:      extern "C" fn(env: *JNIEnvImpl, obj: jobject),
	pub IsSameObject:        extern "C" fn(env: *JNIEnvImpl, obj1: jobject, obj2: jobject) -> jboolean,
	pub NewLocalRef:         extern "C" fn(env: *JNIEnvImpl, lref: jobject) -> jobject,
	pub EnsureLocalCapacity: extern "C" fn(env: *JNIEnvImpl, capacity: jint) -> JniError,

	pub AllocObject:         extern "C" fn(env: *JNIEnvImpl, clazz: jclass) -> jobject,
	pub NewObject:           extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jobject, // DOES NOT WORK!
	pub NewObjectV:          extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jobject, // DOES NOT WORK!
	pub NewObjectA:          extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID, args: *jvalue) -> jobject,

	pub GetObjectClass:      extern "C" fn(env: *JNIEnvImpl, obj: jobject) -> jclass,
	pub IsInstanceOf:        extern "C" fn(env: *JNIEnvImpl, obj: jobject, clazz: jclass) -> jboolean,

	pub GetMethodID:         extern "C" fn(env: *JNIEnvImpl, clazz: jclass, name: *::libc::c_char, sig: *::libc::c_char) -> jmethodID,

	pub CallObjectMethod:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jobject,  // DOES NOT WORK!
	pub CallObjectMethodV:   extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jobject,  // DOES NOT WORK!
	pub CallObjectMethodA:   extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jobject,
	pub CallBooleanMethod:   extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jboolean,  // DOES NOT WORK!
	pub CallBooleanMethodV:  extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jboolean,  // DOES NOT WORK!
	pub CallBooleanMethodA:  extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jboolean,
	pub CallByteMethod:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jbyte,  // DOES NOT WORK!
	pub CallByteMethodV:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jbyte,  // DOES NOT WORK!
	pub CallByteMethodA:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jbyte,
	pub CallCharMethod:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jchar,  // DOES NOT WORK!
	pub CallCharMethodV:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jchar,  // DOES NOT WORK!
	pub CallCharMethodA:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jchar,
	pub CallShortMethod:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jshort,  // DOES NOT WORK!
	pub CallShortMethodV:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jshort,  // DOES NOT WORK!
	pub CallShortMethodA:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jshort,
	pub CallIntMethod:       extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jint,  // DOES NOT WORK!
	pub CallIntMethodV:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jint,  // DOES NOT WORK!
	pub CallIntMethodA:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jint,
	pub CallLongMethod:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jlong,  // DOES NOT WORK!
	pub CallLongMethodV:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jlong,  // DOES NOT WORK!
	pub CallLongMethodA:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jlong,
	pub CallFloatMethod:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jfloat,  // DOES NOT WORK!
	pub CallFloatMethodV:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jfloat,  // DOES NOT WORK!
	pub CallFloatMethodA:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jfloat,
	pub CallDoubleMethod:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jdouble,  // DOES NOT WORK!
	pub CallDoubleMethodV:   extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jdouble,  // DOES NOT WORK!
	pub CallDoubleMethodA:   extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jdouble,
	pub CallVoidMethod:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID),  // DOES NOT WORK!
	pub CallVoidMethodV:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID),  // DOES NOT WORK!
	pub CallVoidMethodA:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue),

	pub CallNonvirtualObjectMethod:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jobject,  // DOES NOT WORK!
	pub CallNonvirtualObjectMethodV:   extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jobject,  // DOES NOT WORK!
	pub CallNonvirtualObjectMethodA:   extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jobject,
	pub CallNonvirtualBooleanMethod:   extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jboolean,  // DOES NOT WORK!
	pub CallNonvirtualBooleanMethodV:  extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jboolean,  // DOES NOT WORK!
	pub CallNonvirtualBooleanMethodA:  extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jboolean,
	pub CallNonvirtualByteMethod:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jbyte,  // DOES NOT WORK!
	pub CallNonvirtualByteMethodV:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jbyte,  // DOES NOT WORK!
	pub CallNonvirtualByteMethodA:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jbyte,
	pub CallNonvirtualCharMethod:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jchar,  // DOES NOT WORK!
	pub CallNonvirtualCharMethodV:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jchar,  // DOES NOT WORK!
	pub CallNonvirtualCharMethodA:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jchar,
	pub CallNonvirtualShortMethod:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jshort,  // DOES NOT WORK!
	pub CallNonvirtualShortMethodV:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jshort,  // DOES NOT WORK!
	pub CallNonvirtualShortMethodA:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jshort,
	pub CallNonvirtualIntMethod:       extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jint,  // DOES NOT WORK!
	pub CallNonvirtualIntMethodV:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jint,  // DOES NOT WORK!
	pub CallNonvirtualIntMethodA:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jint,
	pub CallNonvirtualLongMethod:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jlong,  // DOES NOT WORK!
	pub CallNonvirtualLongMethodV:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jlong,  // DOES NOT WORK!
	pub CallNonvirtualLongMethodA:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jlong,
	pub CallNonvirtualFloatMethod:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jfloat,  // DOES NOT WORK!
	pub CallNonvirtualFloatMethodV:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jfloat,  // DOES NOT WORK!
	pub CallNonvirtualFloatMethodA:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jfloat,
	pub CallNonvirtualDoubleMethod:    extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jdouble,  // DOES NOT WORK!
	pub CallNonvirtualDoubleMethodV:   extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID) -> jdouble,  // DOES NOT WORK!
	pub CallNonvirtualDoubleMethodA:   extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue) -> jdouble,
	pub CallNonvirtualVoidMethod:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID),  // DOES NOT WORK!
	pub CallNonvirtualVoidMethodV:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID),  // DOES NOT WORK!
	pub CallNonvirtualVoidMethodA:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, methodID: jmethodID, args: *jvalue),

	pub GetFieldID:          extern "C" fn(env: *JNIEnvImpl, clazz: jclass, name: *::libc::c_char, sig: *::libc::c_char) -> jfieldID,

	pub GetObjectField:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID) -> jobject,
	pub GetBooleanField:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID) -> jboolean,
	pub GetByteField:        extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID) -> jbyte,
	pub GetCharField:        extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID) -> jchar,
	pub GetShortField:       extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID) -> jshort,
	pub GetIntField:         extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID) -> jint,
	pub GetLongField:        extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID) -> jlong,
	pub GetFloatField:       extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID) -> jfloat,
	pub GetDoubleField:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID) -> jdouble,

	pub SetObjectField:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID, val: jobject),
	pub SetBooleanField:     extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID, val: jboolean),
	pub SetByteField:        extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID, val: jbyte),
	pub SetCharField:        extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID, val: jchar),
	pub SetShortField:       extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID, val: jshort),
	pub SetIntField:         extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID, val: jint),
	pub SetLongField:        extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID, val: jlong),
	pub SetFloatField:       extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID, val: jfloat),
	pub SetDoubleField:      extern "C" fn(env: *JNIEnvImpl, obj: jobject, fieldID: jfieldID, val: jdouble),

	pub GetStaticMethodID:         extern "C" fn(env: *JNIEnvImpl, clazz: jclass, name: *::libc::c_char, sig: *::libc::c_char) -> jmethodID,

	pub CallStaticObjectMethod:    extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jobject,  // DOES NOT WORK!
	pub CallStaticObjectMethodV:   extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jobject,  // DOES NOT WORK!
	pub CallStaticObjectMethodA:   extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID, args: *jvalue) -> jobject,
	pub CallStaticBooleanMethod:   extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jboolean,  // DOES NOT WORK!
	pub CallStaticBooleanMethodV:  extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jboolean,  // DOES NOT WORK!
	pub CallStaticBooleanMethodA:  extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID, args: *jvalue) -> jboolean,
	pub CallStaticByteMethod:      extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jbyte,  // DOES NOT WORK!
	pub CallStaticByteMethodV:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jbyte,  // DOES NOT WORK!
	pub CallStaticByteMethodA:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID, args: *jvalue) -> jbyte,
	pub CallStaticCharMethod:      extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jchar,  // DOES NOT WORK!
	pub CallStaticCharMethodV:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jchar,  // DOES NOT WORK!
	pub CallStaticCharMethodA:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID, args: *jvalue) -> jchar,
	pub CallStaticShortMethod:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jshort,  // DOES NOT WORK!
	pub CallStaticShortMethodV:    extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jshort,  // DOES NOT WORK!
	pub CallStaticShortMethodA:    extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID, args: *jvalue) -> jshort,
	pub CallStaticIntMethod:       extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jint,  // DOES NOT WORK!
	pub CallStaticIntMethodV:      extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jint,  // DOES NOT WORK!
	pub CallStaticIntMethodA:      extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID, args: *jvalue) -> jint,
	pub CallStaticLongMethod:      extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jlong,  // DOES NOT WORK!
	pub CallStaticLongMethodV:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jlong,  // DOES NOT WORK!
	pub CallStaticLongMethodA:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID, args: *jvalue) -> jlong,
	pub CallStaticFloatMethod:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jfloat,  // DOES NOT WORK!
	pub CallStaticFloatMethodV:    extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jfloat,  // DOES NOT WORK!
	pub CallStaticFloatMethodA:    extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID, args: *jvalue) -> jfloat,
	pub CallStaticDoubleMethod:    extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jdouble,  // DOES NOT WORK!
	pub CallStaticDoubleMethodV:   extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID) -> jdouble,  // DOES NOT WORK!
	pub CallStaticDoubleMethodA:   extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID, args: *jvalue) -> jdouble,
	pub CallStaticVoidMethod:      extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID),  // DOES NOT WORK!
	pub CallStaticVoidMethodV:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID),  // DOES NOT WORK!
	pub CallStaticVoidMethodA:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methodID: jmethodID, args: *jvalue),

	pub GetStaticFieldID:          extern "C" fn(env: *JNIEnvImpl, clazz: jclass, name: *::libc::c_char, sig: *::libc::c_char) -> jfieldID,

	pub GetStaticObjectField:      extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID) -> jobject,
	pub GetStaticBooleanField:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID) -> jboolean,
	pub GetStaticByteField:        extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID) -> jbyte,
	pub GetStaticCharField:        extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID) -> jchar,
	pub GetStaticShortField:       extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID) -> jshort,
	pub GetStaticIntField:         extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID) -> jint,
	pub GetStaticLongField:        extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID) -> jlong,
	pub GetStaticFloatField:       extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID) -> jfloat,
	pub GetStaticDoubleField:      extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID) -> jdouble,

	pub SetStaticObjectField:      extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID, val: jobject),
	pub SetStaticBooleanField:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID, val: jboolean),
	pub SetStaticByteField:        extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID, val: jbyte),
	pub SetStaticCharField:        extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID, val: jchar),
	pub SetStaticShortField:       extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID, val: jshort),
	pub SetStaticIntField:         extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID, val: jint),
	pub SetStaticLongField:        extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID, val: jlong),
	pub SetStaticFloatField:       extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID, val: jfloat),
	pub SetStaticDoubleField:      extern "C" fn(env: *JNIEnvImpl, clazz: jclass, fieldID: jfieldID, val: jdouble),

	pub NewString:           extern "C" fn(env: *JNIEnvImpl, unicode: *jchar, len: jsize) -> jstring,
	pub GetStringLength:     extern "C" fn(env: *JNIEnvImpl, strg: jstring) -> jsize,
	pub GetStringChars:      extern "C" fn(env: *JNIEnvImpl, strg: jstring, isCopy: *mut jboolean) -> *jchar,
	pub ReleaseStringChars:  extern "C" fn(env: *JNIEnvImpl, strg: jstring, chars: *jchar),

	pub NewStringUTF:        extern "C" fn(env: *JNIEnvImpl, utf: *::libc::c_char) -> jstring,
	pub GetStringUTFLength:  extern "C" fn(env: *JNIEnvImpl, strg: jstring) -> jsize,
	pub GetStringUTFChars:   extern "C" fn(env: *JNIEnvImpl, strg: jstring, isCopy: *mut jboolean) -> *::libc::c_char,
	pub ReleaseStringUTFChars: extern "C" fn(env: *JNIEnvImpl, strg: jstring, chars: *::libc::c_char),

	pub GetArrayLength:      extern "C" fn(env: *JNIEnvImpl, array: jarray) -> jsize,

	pub NewObjectArray:      extern "C" fn(env: *JNIEnvImpl, len: jsize, clazz: jclass, init: jobject) -> jobjectArray,
	pub GetObjectArrayElement: extern "C" fn(env: *JNIEnvImpl, array: jobjectArray, index: jsize) -> jobject,
	pub SetObjectArrayElement: extern "C" fn(env: *JNIEnvImpl, array: jobjectArray, index: jsize, val: jobject),

	pub NewBooleanArray:     extern "C" fn(env: *JNIEnvImpl, len: jsize) -> jbooleanArray,
	pub NewByteArray:        extern "C" fn(env: *JNIEnvImpl, len: jsize) -> jbyteArray,
	pub NewCharArray:        extern "C" fn(env: *JNIEnvImpl, len: jsize) -> jcharArray,
	pub NewShortArray:       extern "C" fn(env: *JNIEnvImpl, len: jsize) -> jshortArray,
	pub NewIntArray:         extern "C" fn(env: *JNIEnvImpl, len: jsize) -> jintArray,
	pub NewLongArray:        extern "C" fn(env: *JNIEnvImpl, len: jsize) -> jlongArray,
	pub NewFloatArray:       extern "C" fn(env: *JNIEnvImpl, len: jsize) -> jfloatArray,
	pub NewDoubleArray:      extern "C" fn(env: *JNIEnvImpl, len: jsize) -> jdoubleArray,

	pub GetBooleanArrayElements: extern "C" fn(env: *JNIEnvImpl, array: jbooleanArray, isCopy: *mut jboolean) -> *jboolean,
	pub GetByteArrayElements:    extern "C" fn(env: *JNIEnvImpl, array: jbyteArray, isCopy: *mut jboolean) -> *jbyte,
	pub GetCharArrayElements:    extern "C" fn(env: *JNIEnvImpl, array: jcharArray, isCopy: *mut jboolean) -> *jchar,
	pub GetShortArrayElements:   extern "C" fn(env: *JNIEnvImpl, array: jshortArray, isCopy: *mut jboolean) -> *jshort,
	pub GetIntArrayElements:     extern "C" fn(env: *JNIEnvImpl, array: jintArray, isCopy: *mut jboolean) -> *jint,
	pub GetLongArrayElements:    extern "C" fn(env: *JNIEnvImpl, array: jlongArray, isCopy: *mut jboolean) -> *jlong,
	pub GetFloatArrayElements:   extern "C" fn(env: *JNIEnvImpl, array: jfloatArray, isCopy: *mut jboolean) -> *jfloat,
	pub GetDoubleArrayElements:  extern "C" fn(env: *JNIEnvImpl, array: jdoubleArray, isCopy: *mut jboolean) -> *jdouble,

	pub ReleaseBooleanArrayElements: extern "C" fn(env: *JNIEnvImpl, array: jbooleanArray, elems: *mut jboolean, mode: JniReleaseArrayElementsMode),
	pub ReleaseByteArrayElements:    extern "C" fn(env: *JNIEnvImpl, array: jbyteArray, elems: *mut jbyte, mode: JniReleaseArrayElementsMode),
	pub ReleaseCharArrayElements:    extern "C" fn(env: *JNIEnvImpl, array: jcharArray, elems: *mut jchar, mode: JniReleaseArrayElementsMode),
	pub ReleaseShortArrayElements:   extern "C" fn(env: *JNIEnvImpl, array: jshortArray, elems: *mut jshort, mode: JniReleaseArrayElementsMode),
	pub ReleaseIntArrayElements:     extern "C" fn(env: *JNIEnvImpl, array: jintArray, elems: *mut jint, mode: JniReleaseArrayElementsMode),
	pub ReleaseLongArrayElements:    extern "C" fn(env: *JNIEnvImpl, array: jlongArray, elems: *mut jlong, mode: JniReleaseArrayElementsMode),
	pub ReleaseFloatArrayElements:   extern "C" fn(env: *JNIEnvImpl, array: jfloatArray, elems: *mut jfloat, mode: JniReleaseArrayElementsMode),
	pub ReleaseDoubleArrayElements:  extern "C" fn(env: *JNIEnvImpl, array: jdoubleArray, elems: *mut jdouble, mode: JniReleaseArrayElementsMode),

	pub GetBooleanArrayRegion: extern "C" fn(env: *JNIEnvImpl, array: jbooleanArray, start: jsize, l: jsize, buf: *mut jboolean),
	pub GetByteArrayRegion:    extern "C" fn(env: *JNIEnvImpl, array: jbyteArray, start: jsize, l: jsize, buf: *mut jbyte),
	pub GetCharArrayRegion:    extern "C" fn(env: *JNIEnvImpl, array: jcharArray, start: jsize, l: jsize, buf: *mut jchar),
	pub GetShortArrayRegion:   extern "C" fn(env: *JNIEnvImpl, array: jshortArray, start: jsize, l: jsize, buf: *mut jshort),
	pub GetIntArrayRegion:     extern "C" fn(env: *JNIEnvImpl, array: jintArray, start: jsize, l: jsize, buf: *mut jint),
	pub GetLongArrayRegion:    extern "C" fn(env: *JNIEnvImpl, array: jlongArray, start: jsize, l: jsize, buf: *mut jlong),
	pub GetFloatArrayRegion:   extern "C" fn(env: *JNIEnvImpl, array: jfloatArray, start: jsize, l: jsize, buf: *mut jfloat),
	pub GetDoubleArrayRegion:  extern "C" fn(env: *JNIEnvImpl, array: jdoubleArray, start: jsize, l: jsize, buf: *mut jdouble),

	pub SetBooleanArrayRegion: extern "C" fn(env: *JNIEnvImpl, array: jbooleanArray, start: jsize, l: jsize, buf: *jboolean),
	pub SetByteArrayRegion:    extern "C" fn(env: *JNIEnvImpl, array: jbyteArray, start: jsize, l: jsize, buf: *jbyte),
	pub SetCharArrayRegion:    extern "C" fn(env: *JNIEnvImpl, array: jcharArray, start: jsize, l: jsize, buf: *jchar),
	pub SetShortArrayRegion:   extern "C" fn(env: *JNIEnvImpl, array: jshortArray, start: jsize, l: jsize, buf: *jshort),
	pub SetIntArrayRegion:     extern "C" fn(env: *JNIEnvImpl, array: jintArray, start: jsize, l: jsize, buf: *jint),
	pub SetLongArrayRegion:    extern "C" fn(env: *JNIEnvImpl, array: jlongArray, start: jsize, l: jsize, buf: *jlong),
	pub SetFloatArrayRegion:   extern "C" fn(env: *JNIEnvImpl, array: jfloatArray, start: jsize, l: jsize, buf: *jfloat),
	pub SetDoubleArrayRegion:  extern "C" fn(env: *JNIEnvImpl, array: jdoubleArray, start: jsize, l: jsize, buf: *jdouble),

	pub RegisterNatives:     extern "C" fn(env: *JNIEnvImpl, clazz: jclass, methods: *JNINativeMethod, nMethods: jint) -> JniError,
	pub UnregisterNatives:   extern "C" fn(env: *JNIEnvImpl, clazz: jclass) -> JniError,

	pub MonitorEnter:        extern "C" fn(env: *JNIEnvImpl, obj: jobject) -> JniError,
	pub MonitorExit:         extern "C" fn(env: *JNIEnvImpl, obj: jobject) -> JniError,

	pub GetJavaVM:           extern "C" fn(env: *JNIEnvImpl, vm: *mut *JavaVMImpl) -> JniError,

	pub GetStringRegion:     extern "C" fn(env: *JNIEnvImpl, st: jstring, start: jsize, len: jsize, buf: *mut jchar),
	pub GetStringUTFRegion:  extern "C" fn(env: *JNIEnvImpl, st: jstring, start: jsize, len: jsize, buf: *mut ::libc::c_char),

	pub GetPrimitiveArrayCritical:     extern "C" fn(env: *JNIEnvImpl, array: jarray, isCopy: *mut jboolean),
	pub ReleasePrimitiveArrayCritical: extern "C" fn(env: *JNIEnvImpl, array: jarray, carray: *jvoid, mode: JniReleaseArrayElementsMode),

	pub GetStringCritical:     extern "C" fn(env: *JNIEnvImpl, string: jstring, isCopy: *mut jboolean) -> *jchar,
	pub ReleaseStringCritical: extern "C" fn(env: *JNIEnvImpl, string: jstring, cstring: *jchar),

	pub NewWeakGlobalRef:    extern "C" fn(env: *JNIEnvImpl, rf: jobject) -> jweak,
	pub DeleteWeakGlobalRef: extern "C" fn(env: *JNIEnvImpl, rf: jweak),

	pub ExceptionCheck:      extern "C" fn(env: *JNIEnvImpl) -> jboolean,

	pub NewDirectByteBuffer:     extern "C" fn(env: *JNIEnvImpl, address: *mut jvoid, capacity: jlong) -> jobject,
	pub GetDirectBufferAddress:  extern "C" fn(env: *JNIEnvImpl, buf: jobject) -> *mut jvoid,
	pub GetDirectBufferCapacity: extern "C" fn(env: *JNIEnvImpl, buf: jobject) -> jlong,

	pub GetObjectRefType:    extern "C" fn(env: *JNIEnvImpl, obj: jobject) -> jobjectRefType
}

pub type JNIEnvImpl = *JNINativeInterface;


#[link(name = "jvm")]
extern "C" {
	pub fn JNI_CreateJavaVM(vm: *mut *JavaVMImpl, env: *mut *JNIEnvImpl, args: *JavaVMInitArgsImpl) -> JniError;
}


pub struct JNINativeMethod {
	name: *::libc::c_char,
	signature: *::libc::c_char,
	fnPtr: *jvoid
}

pub enum jobjectRefType {
	JNIInvalidRefType    = 0,
	JNILocalRefType      = 1,
	JNIGlobalRefType     = 2,
	JNIWeakGlobalRefType = 3
}


pub struct JavaVMOptionImpl {
	pub optionString: *libc::c_char,
	pub extraInfo: *jvoid
}

pub struct JavaVMInitArgsImpl {
	pub version: JniVersion,
	pub nOptions: jint,
	pub options: *JavaVMOptionImpl,
	pub ignoreUnrecognized: jboolean
}

pub struct JavaVMAttachArgsImpl {
	pub version: JniVersion,
	pub name: *libc::c_char,
	pub group: jobject
}
