//! 该模块定义通用的异步/远程接口调用结果。
use std::error::Error;
use std::fmt::{Debug, Display};

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
        let ret: ApiResp;
        match self {
            Ok(r) => {
                ret = r;
            }
            Err(e) => {
                error!("{} {:?}", err_log, e);
                ret = ApiResp::error(-1, e.to_string());
            }
        }
        serde_json::to_string(&ret).unwrap()
    }
}
