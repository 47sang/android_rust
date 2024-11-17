use jni::objects::{JClass, JString};
use jni::sys::jstring;
use jni::JNIEnv;
use tokio;

#[no_mangle]
pub extern "C" fn Java_com_galigeigei_rustapplication_MainActivity_greet(
    mut env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    // 将 Java 字符串转为 Rust 字符串
    let input: String = env.get_string(&input).expect("Invalid string").into();
    let output = format!(
        "集成编译,rust调用方法太牛了!!!这次是小米手机调用的 {}!",
        input
    );
    println!("我打印输出了一行日志,可以看到吗?");

    // 返回新的 Java 字符串
    env.new_string(output)
        .expect("无法创建 Java 字符串")
        .into_raw()
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
    env.new_string(output)
        .expect("无法创建 Java 字符串")
        .into_raw()
}

const PC_API_URL: &str = "http://192.168.5.18:8080";

#[no_mangle]
pub extern "C" fn Java_com_galigeigei_rustapplication_MainActivity_getApi<'a>(
    env: JNIEnv<'a>,
    _class: JClass<'a>,
) -> jstring {

    let result = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(get_http_response());

    let result = result.unwrap_or_else(|err| err.to_string());

    env.new_string(result).expect("无法创建 Java 字符串").into_raw()
}

// 修正返回值为 Result<String, reqwest::Error>
async fn get_http_response() -> Result<String, reqwest::Error> {
    // 创建 HTTP 客户端
    let client = reqwest::Client::new();

    // 发起 HTTP 请求并返回结果
    let response = client.get(PC_API_URL).send().await?;
    let text = response.text().await?;
    Ok(text)
}
