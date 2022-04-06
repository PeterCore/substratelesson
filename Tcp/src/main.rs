use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    // 创建服务
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    // 给服务创建 多个发送者，多个接收者通道
    let (tx, _) = broadcast::channel(10);
    loop {
        //接受新的连接，创建新的Tcp
        let (mut socket, addr) = listener.accept().await.unwrap();
        // clone 发送者
        let tx = tx.clone();
        // 创建新的接收者
        let mut rx = tx.subscribe();
        //生成异步并 转移上下文所有权
        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
            loop {
                tokio::select! {
                    //读取内存中的line 发送给所有活动的接收者
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() => {
                        //接收message 写入内存中。
                        let (msg, other_addr) = result.unwrap();
                        if addr != other_addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
