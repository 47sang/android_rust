package com.galigeigei.rustapplication

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.galigeigei.rustapplication.ui.theme.RustApplicationTheme

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            RustApplicationTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    Column {
                        Greeting(
                            name = "Android", modifier = Modifier.padding(innerPadding)
                        )
                        //按钮
                        Button(onClick = {
                            val sayOut = RustLib.say("你想说的话")
                            Log.d("RustFFI", sayOut)
                        }) {
                            Text("我是一个按钮")
                        }

                        //按钮
                        Button(onClick = {
                            // 在协程中调用异步方法
                            CoroutineScope(Dispatchers.IO).launch {
                                val apiStr = RustLib.getApi() // 在后台线程运行
                                withContext(Dispatchers.Main) {
                                    // 切换到主线程更新 UI 或日志
                                    Log.d("RustFFI", apiStr)
                                }
                            }
                        }) {
                            Text("发起一个网络请求")
                        }

                        // 调用本地方法
                        val greeting = RustLib.greet("Rust")
                        Log.d("RustFFI", "输出生成的rust调用函数的结果: $greeting")
                    }
                }
            }
        }
    }
}

@Composable
fun Greeting(name: String, modifier: Modifier = Modifier) {
    Text(
        text = "Hello $name!", modifier = modifier
    )
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    RustApplicationTheme {
        Greeting("Android")
    }
}