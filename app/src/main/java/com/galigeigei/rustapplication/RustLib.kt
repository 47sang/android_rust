package com.galigeigei.rustapplication

object RustLib {
    // 加载 Rust 生成的库
    init {
        System.loadLibrary("rust_c")
    }

    // 声明本地方法
    external fun greet(input: String): String
    external fun say(input: String): String
    external fun getApi(): String
    
}