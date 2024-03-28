# 基础类
| 版本|作者 | 描述 |
| :----: | :----: | :----: |
| V0.1.0 | 2024-03-26 | 初始化版本 |

## 任务列表
- [X] md5
- [ ] base64
- [ ] jwt
- [ ] SHA256

## md5
主要是进行md5加密

### 测试程序

```rust
use sunnyutil::md5;

fn main(){
    println!("md5 {}",md5::md5("hello world!"));
}
```