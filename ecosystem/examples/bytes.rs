use std::any::type_name_of_val;

use bytes::{BufMut as _, BytesMut};

fn main() -> anyhow::Result<()> {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice("hello world".as_bytes());
    println!("{:?}", buf);

    buf.put(&b"goodbye world"[..]);
    println!("{:?}", buf);

    let a = buf.split();
    let mut b = a.freeze();
    println!("{:?}", b);

    let c = b.split_to(12);
    println!("{:?}", c);

    println!("{:?}", b);

    let byte_literal = b"hello world";
    // 借用字节字符串字面量
    let borrowed_literal = &b"hello world";

    // 打印类型
    println!("byte_literal 的类型是: {}", type_name_of_val(byte_literal));
    println!(
        "borrowed_literal 的类型是: {}",
        type_name_of_val(borrowed_literal)
    );

    Ok(())
}
