use tiberius::{AuthMethod, Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

pub async fn client() -> Result<Client<Compat<TcpStream>>,String> {
    let mut config = Config::new();
    config.host("192.168.10.254");
    config.port(1455);
    config.database("CY_MESDATA");
    config.authentication(AuthMethod::sql_server("cytest", "cytest"));
    // config.authentication(AuthMethod::sql_server("hztest", "hztest"));
    config.trust_cert();
    // let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
    
    let tcp = TcpStream::connect(config.get_addr()).await;
    match tcp {
        Ok(tcp) => {
            tcp.set_nodelay(true).unwrap();
            let client = tiberius::Client::connect(config.clone(), tcp.compat_write()).await;
            match client {
                Ok(client) => Ok(client),
                Err(_) => {
                    config.port(1433);
                    config.host("127.0.0.1");
                    let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
                    tcp.set_nodelay(true).unwrap();
                    let client = tiberius::Client::connect(config, tcp.compat_write())
                        .await ;
                    match client {
                        Ok(client) => {
                            Ok(client)
                        },
                        Err(_) => {
                            Err("数据库连接失败".to_string())
                        },
                    }
                },
            }
        },
        Err(_) => {
            config.port(1433);
            config.host("127.0.0.1");
            let tcp = TcpStream::connect(config.get_addr()).await.unwrap();
            tcp.set_nodelay(true).unwrap();
            let client = tiberius::Client::connect(config, tcp.compat_write())
                .await;
            match client {
                Ok(client) => {
                    Ok(client)
                },
                Err(_) => {
                    Err("数据库连接失败".to_string())
                },
            }
        },
   
    }
}
    
