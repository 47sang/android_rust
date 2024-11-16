use std::fmt::Display;
use std::str::FromStr;
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

const PC_API_URL: &str = "http://192.168.31.225:8080";

#[no_mangle]
pub  async extern "C" fn Java_com_galigeigei_rustapplication_MainActivity_getApi<'a>(
    env: JNIEnv<'a>,
    _class: JClass<'a>,
) -> jstring {
    // 创建 HTTP 客户端
    let client = reqwest::Client::new();

    // 发起 HTTP 请求
    let result = client
        .get(PC_API_URL)
        .send()
        .await;

    match result {
        Ok(response) => {
            let json_value = response.json::<serde_json::Value>().await;
            match json_value {
                Ok(json) => env.new_string(json.to_string()).expect("创建 Java 字符串失败").into_raw(),
                Err(_) => env.new_string("解析 JSON 失败").expect("创建 Java 字符串失败").into_raw(),
            }
        }
        Err(_) => env.new_string("HTTP 请求失败").expect("创建 Java 字符串失败").into_raw(),
    }
}