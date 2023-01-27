//! 该模块定义通用的异步/远程接口调用结果。
use std::error::Error;
use std::fmt::{Debug, Display};
use log::error;
use serde::{Serialize,Deserialize};

/// API接口响应数据结构。
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResp {
    /// 执行是否成功
    success: bool,
    /// 响应代码
    code: i32,
    /// 响应附带消息，通常是错误提示信息。
    message: String,
    /// 响应数据。
    data: Option<serde_json::Value>,
}

impl ApiResp {
    pub fn is_success(&self) -> bool { self.success }

    pub fn get_code(&self) -> i32 { self.code }

    pub fn get_message(&self) -> &String { &self.message }

    pub fn get_data(&self) -> &Option<serde_json::Value> { &self.data }

    pub fn to_json(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(json) => json,
            Err(e) => {
                error!("序列化json字符串时出错！{}", e);
                let err_resp = ApiResp::error(-1, "处理响应结果时出错！".to_string());
                serde_json::to_string(&err_resp).unwrap()
            }
        }
    }
}

impl ApiResp {
    /// 构造一个成功的响应对象。
    ///
    /// # Arguments
    ///
    /// * `data`: 业务数据。
    ///
    /// returns: ApiResp
    ///
    /// # Examples
    ///
    /// ```
    /// use api_resp::ApiResp;
    /// use serde_json::json;
    /// let data = vec![1,1,3,5];
    /// let resp = ApiResp::success(json!(data));
    /// ```
    pub fn success(data: serde_json::Value) -> ApiResp {
        ApiResp {
            success: true,
            code: 0,
            message: "".to_string(),
            data: Some(data),
        }
    }

    /// 构造一个成功的简单响应对象，不带任何消息。
    ///
    /// returns: ApiResp 返回成功响应。
    ///
    /// # Examples
    ///
    /// ```
    /// use api_resp::ApiResp;
    /// let resp = ApiResp::suc();
    /// ```
    pub fn suc() -> ApiResp {
        ApiResp {
            success: true,
            code: 0,
            message: "".to_string(),
            data: None,
        }
    }


    /// 构造一个失败的响应对象。
    ///
    /// # Arguments
    ///
    /// * `code`: 失败代码。根据具体的业务接口约定取值列表。
    /// * `message`: 失败信息。
    ///
    /// returns: ApiResp
    ///
    /// # Examples
    ///
    /// ```
    /// use api_resp::ApiResp;
    /// let resp = ApiResp::fail(-1, String::from("查询信息失败，原因:..."));
    /// ```
    pub fn error(code: i32, message: String) -> ApiResp {
        ApiResp {
            success: false,
            code,
            message,
            data: None,
        }
    }
}

/// 简写的接口返回数据结构定义。
pub type DaoResult = Result<ApiResp, Box<dyn Error>>;

/// 将API调用结果转换为对外数据形式的特性声明。
pub trait TransformResult {
    /// 将API结果转换为JSON字符串。
    ///
    /// # Arguments
    ///
    /// * `err_log`: 客制化的出错日志信息。
    ///
    /// returns: String 返回JSON字符串。
    ///
    /// # Examples
    ///
    /// ```
    /// use serde::{Deserialize, Serialize};
    /// use std::fmt::{Debug, Display};
    /// use api_resp::TransformResult;
    /// #[derive(Debug, Serialize, Deserialize)]
    /// struct Dept {
    ///     id: Option<String>,
    ///     pid: Option<String>,
    ///     other_attr: Option<i32>
    /// }
    ///
    /// impl TransformResult for Dept {
    ///     fn to_json_str<T>(self, err_log: T) -> String where T: Debug + Display {
    ///         serde_json::to_string(&self).unwrap()
    ///     }
    /// }
    ///
    /// let d = Dept {id: Some("01".to_string()), pid: None, other_attr: Some(10)};
    ///
    /// let json_str = d.to_json_str("执行出错".to_string());
    /// println!("json_str: {}", json_str);
    /// ```
    fn to_json_str<T>(self, err_log: T) -> String where T: Debug + Display;
}

impl TransformResult for DaoResult {
    fn to_json_str<T>(self, err_log: T) -> String where T: Debug + Display {
        let ret: ApiResp = match self {
            Ok(r) => r,
            Err(e) => {
                error!("{} {:?}", err_log, e);
                ApiResp::error(-1, e.to_string())
            }
        };
        serde_json::to_string(&ret).unwrap()
    }
}

/// 回滚当前的事务后退出当前函数，返回包含通用错误信息的结果对象。
#[macro_export]
macro_rules! rollback {
    ($resp: expr, $tx: expr, $code: expr) => {
        if let Err(e) = $resp {
            $tx.rollback().await?;
            return Ok(ApiResp::error($code, e.to_string()));
        }
    };
}

/// 当出现错误或更新记录数未0时，回滚当前的事务后退出当前函数，返回包含通用错误信息的结果对象。
#[macro_export]
macro_rules! rollback_for_no_match {
    ($resp: expr, $tx: expr, $code: expr) => {
        match $resp {
            Err(e) => {
                $tx.rollback().await?;
                return Ok(ApiResp::error($code, e.to_string()));
            },
            Ok(r) if r.rows_affected == 0 => {
                $tx.rollback().await?;
                return Ok(ApiResp::error($code, "未匹配到目标记录".to_string()));
            },
            _ => {}
        }
    };
}

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
