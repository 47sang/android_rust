use jni::objects::{JString, JClass};
use jni::sys::jstring;
use jni::JNIEnv;

#[no_mangle]
pub extern "C" fn Java_com_galigeigei_rustapplication_MainActivity_greet(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    // 将 Java 字符串转为 Rust 字符串
    let input: String = env.get_string(&input).expect("Invalid string").into();
    let output = format!("集成编译,rust调用方法太牛了!!!这次是小米手机调用的 {}!", input);
    println!("我打印输出了一行日志,可以看到吗?");

    // 返回新的 Java 字符串
    env.new_string(output).expect("无法创建 Java 字符串").into_raw()
}


#[no_mangle]
pub extern "C" fn Java_com_galigeigei_rustapplication_MainActivity_say(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    // 将 Java 字符串转为 Rust 字符串
    let input: String = env.get_string(&input).expect("Invalid string").into();
    let output = format!("我的名字叫:{}", input);
    println!("我打印输出了一行日志,可以看到吗?打印与say方法被调用了");

    // 返回新的 Java 字符串
    env.new_string(output).expect("无法创建 Java 字符串").into_raw()
}