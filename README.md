该模块为远程/异步调用API的返回结果，定义通用的封装结构和基本方法。

用法示例：
```rust
use some_service::user_edit::{new_user, User, UserExt, UserRole};
use api_resp::{DaoResult, TransformResult};

#[tauri::command]
pub async fn add_new_user(mut user: User, mut ext: UserExt, mut roles: Vec<UserRole>) -> String {
    let result: DaoResult = new_user(&mut user, &mut ext, &mut roles).await;
    result.to_json_str("新增用户时出错")
}
```
