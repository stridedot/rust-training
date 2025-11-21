use bytes::BytesMut;
use futures::SinkExt as _;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};

use crate::backend::Backend;

use crate::cmd::{Cmd, CmdExecutor as _};
use crate::resp::frame::RespFrame;
use crate::resp::{RespDecode as _, RespEncode as _, RespError};

struct Request {
    frame: RespFrame,
    backend: Backend,
}

struct Response {
    frame: RespFrame,
}

struct RespFrameCodec;

impl Encoder<RespFrame> for RespFrameCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: RespFrame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let encoded = item.encode();
        dst.extend_from_slice(&encoded);
        Ok(())
    }
}

impl Decoder for RespFrameCodec {
    type Item = RespFrame;
    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        match RespFrame::decode(src) {
            Ok(frame) => Ok(Some(frame)),
            Err(RespError::Incomplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

pub async fn handle_stream(socket: TcpStream, backend: &Backend) -> anyhow::Result<()> {
    let codec = RespFrameCodec;
    let mut framed = Framed::new(socket, codec);

    // 举例
    // *3\r\n
    // $3\r\nSET\r\n
    // $3\r\nkey\r\n
    // $5\r\nvalue\r\n
    //
    // 收到的 frame （解析后）是：
    //
    // RespFrame::Array(vec![
    //     RespFrame::BulkString("SET"),
    //     RespFrame::BulkString("key"),
    //     RespFrame::BulkString("value"),
    // ])
    //
    // framed.next() 消耗的是整个 RESP 帧（整个数组）
    while let Some(result) = framed.next().await {
        match result {
            Ok(frame) => {
                eprintln!("Received frame: {:?}", frame);

                let request = Request {
                    frame,
                    backend: backend.clone(),
                };
                let resp = handle_request(request).await?;
                framed.send(resp.frame).await?;
            }
            Err(e) => {
                eprintln!("Decode error: {:?}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}

async fn handle_request(request: Request) -> anyhow::Result<Response> {
    let (frame, backend) = (request.frame, request.backend);
    let cmd = Cmd::try_from(frame)?;
    let resp = cmd.execute(&backend)?;
    Ok(Response { frame: resp })
}
