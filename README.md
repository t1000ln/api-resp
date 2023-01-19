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
#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    #[derive(Debug, Serialize, Deserialize)]
    struct PingPang {
        color: String,
        weight: f64,
    }

    #[test]
    fn test_resp() {
        // 成功结果，没有业务数据。
        let suc_json = ApiResp::suc().to_json();
        println!("suc_json: {}", suc_json);
        let orig_suc: ApiResp = serde_json::from_str(suc_json.as_str()).unwrap();
        assert!(orig_suc.is_success());

        // 成功结果，带有业务数据。
        let vals = vec![
            PingPang {color: "white".to_string(), weight: 10.0},
            PingPang {color: "yellow".to_string(), weight: 11.5},
        ];
        let suc_data = ApiResp::success(json!(vals)).to_json();
        println!("suc_data: {}", suc_data);
        let orig_suc_data: ApiResp = serde_json::from_str(suc_data.as_str()).unwrap();
        assert!(orig_suc_data.is_success());

        // 失败结果。
        let fail_json = ApiResp::error(-1, String::from("交易出错了")).to_json();
        println!("fail_json: {}", fail_json);
        let orig_fail: ApiResp = serde_json::from_str(fail_json.as_str()).unwrap();
        assert!(!orig_fail.is_success());
    }
}
```
执行上面的测试方法将打印出信息：
```json
suc_json: {"success":true,"code":0,"message":"","data":null}
suc_data: {"success":true,"code":0,"message":"","data":[{"color":"white","weight":10.0},{"color":"yellow","weight":11.5}]}
fail_json: {"success":false,"code":-1,"message":"交易出错了","data":null}
```
