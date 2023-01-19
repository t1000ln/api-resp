该模块为远程/异步调用API的返回结果，定义通用的封装结构和基本方法。

返回的数据是`JSON`格式的，结构示例：
```json
{
  "success": true,
  "code": 0,
  "message": "",
  "data": []
}
```
四个属性简要说明：
- `success` 表示调用是否成功。
- `code` 成功为`0`,失败为非`0`的整数值。
- `message` 在失败时提供简要的说明信息。
- `data` 返回的业务数据，也是`JSON`格式。


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
