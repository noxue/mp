## 用法

```rust
fn mp_handler(data: String, query: Query<QueryData>) -> String {
    if !check_signature(
        "asdfsadfa",
        &query.signature,
        &query.timestamp,
        &query.nonce,
    ) {
        return "error".to_string();
    }
    debug!("收到数据: {}", data);
    let msg_data: Msg = data.try_into().unwrap();
    debug!("解析数据: {:?}", msg_data);
    if let Msg::TextMsg(msg) = msg_data {
        let data = reply_to_xml(
            Reply::Text(Text{
                content:msg.content,
            }),
            &msg.to_user_name,
            &msg.from_user_name,
        ).unwrap();
        debug!("回复数据: {}", data);
        return data;
    }
    "hello".to_string()
}
```