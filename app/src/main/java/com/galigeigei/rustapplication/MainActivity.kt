package com.galigeigei.rustapplication

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.galigeigei.rustapplication.ui.theme.RustApplicationTheme

class MainActivity : ComponentActivity() {

    // 加载 Rust 生成的库
    companion object {
        init {
            System.loadLibrary("rust_c") // 替换为 Rust 项目生成的库名
        }
    }

    // 声明本地方法
    private external fun greet(input: String): String
    private external fun say(input: String): String


    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            RustApplicationTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    Greeting(
                        name = "Android",
                        modifier = Modifier.padding(innerPadding)
                    )
                    //按钮
                    Button(onClick = {
                       val sayOut =  say("你想说的话")
                        Log.d("RustFFI", sayOut) //
                    }) {
                        Text("我是一个按钮")
                    }


                    // 调用本地方法
                    val greeting = greet("Rust")
                    Log.d("RustFFI", "输出生成的rust调用函数的结果: $greeting") // 输出 "Greeting: Hello, Rust!"
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!",
        modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    RustApplicationTheme {
        Greeting("Android")

    }
}