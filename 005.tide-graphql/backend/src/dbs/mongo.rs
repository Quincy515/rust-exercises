use crate::util::constant::CFG;

use mongodb::{options::ClientOptions, Client, Database};

pub struct DataSource {
  client: Client,
  pub db_custer: Database,
}

#[allow(dead_code)]
impl DataSource {
  pub async fn client(&self) -> Client {
    self.client.clone()
  }

  pub async fn init() -> DataSource {
    // 解析 MongoDB 连接字符串到 options 结构体中
    let mut client_options = ClientOptions::parse(CFG.get("MONGODB_URI").unwrap())
      .await
      .expect("Failed to parse mongodb options!");
    // 可选：自定义一个 option 选项
    client_options.app_name = Some("tide-graphql-mongodb".to_string());

    // 客户端句柄
    let client = Client::with_options(client_options).expect("Failed to initialize mongodb!");

    // 数据库句柄
    let db_custer = client.database(CFG.get("MONGODB_CUSTER").unwrap());

    // 返回值 mongodb datasource
    DataSource {
      client: client,
      db_custer: db_custer,
    }
  }
}
