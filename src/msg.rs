use serde_xml_rs::from_str;
use std::str::FromStr;

/*
<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[fromUser]]></FromUserName>
  <CreateTime>1348831860</CreateTime>
  <MsgType><![CDATA[text]]></MsgType>
  <Content><![CDATA[this is a test]]></Content>
  <MsgId>1234567890123456</MsgId>
  <MsgDataId>xxxx</MsgDataId>
  <Idx>xxxx</Idx>
</xml>
 */
#[derive(serde::Deserialize, Debug)]
pub struct TextMsg {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "Content")]
    pub content: String,
    #[serde(rename = "MsgId")]
    pub msg_id: u64,
    #[serde(default)]
    #[serde(rename = "MsgDataId")]
    pub msg_data_id: String,
    #[serde(default)]
    #[serde(rename = "Idx")]
    pub idx: String,
}

/*
<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[fromUser]]></FromUserName>
  <CreateTime>1348831860</CreateTime>
  <MsgType><![CDATA[image]]></MsgType>
  <PicUrl><![CDATA[this is a url]]></PicUrl>
  <MediaId><![CDATA[media_id]]></MediaId>
  <MsgId>1234567890123456</MsgId>
   <MsgDataId>xxxx</MsgDataId>
  <Idx>xxxx</Idx>
</xml>

*/
#[derive(serde::Deserialize, Debug)]
pub struct ImageMsg {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "PicUrl")]
    pub pic_url: String,
    #[serde(rename = "MediaId")]
    pub media_id: String,
    #[serde(rename = "MsgId")]
    pub msg_id: u64,
    #[serde(default)]
    #[serde(rename = "MsgDataId")]
    pub msg_data_id: String,
    #[serde(default)]
    #[serde(rename = "Idx")]
    pub idx: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct VoiceMsg {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "MediaId")]
    pub media_id: String,
    #[serde(rename = "Format")]
    pub format: String,
    #[serde(default)]
    #[serde(rename = "Recognition")]
    pub recognition: String,
    #[serde(rename = "MsgId")]
    pub msg_id: u64,
    #[serde(default)]
    #[serde(rename = "MsgDataId")]
    pub msg_data_id: String,
    #[serde(default)]
    #[serde(rename = "Idx")]
    pub idx: String,
}

/*
<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[fromUser]]></FromUserName>
  <CreateTime>1357290913</CreateTime>
  <MsgType><![CDATA[video]]></MsgType>
  <MediaId><![CDATA[media_id]]></MediaId>
  <ThumbMediaId><![CDATA[thumb_media_id]]></ThumbMediaId>
  <MsgId>1234567890123456</MsgId>
   <MsgDataId>xxxx</MsgDataId>
  <Idx>xxxx</Idx>
</xml>
 */
#[derive(serde::Deserialize, Debug)]
pub struct VideoMsg {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "MediaId")]
    pub media_id: String,
    #[serde(rename = "ThumbMediaId")]
    pub thumb_media_id: String,
    #[serde(rename = "MsgId")]
    pub msg_id: u64,
    #[serde(default)]
    #[serde(rename = "MsgDataId")]
    pub msg_data_id: String,
    #[serde(default)]
    #[serde(rename = "Idx")]
    pub idx: String,
}

/*
<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[fromUser]]></FromUserName>
  <CreateTime>1357290913</CreateTime>
  <MsgType><![CDATA[shortvideo]]></MsgType>
  <MediaId><![CDATA[media_id]]></MediaId>
  <ThumbMediaId><![CDATA[thumb_media_id]]></ThumbMediaId>
  <MsgId>1234567890123456</MsgId>
   <MsgDataId>xxxx</MsgDataId>
  <Idx>xxxx</Idx>
</xml>
 */
#[derive(serde::Deserialize, Debug)]
pub struct ShortVideoMsg {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "MediaId")]
    pub media_id: String,
    #[serde(rename = "ThumbMediaId")]
    pub thumb_media_id: String,
    #[serde(rename = "MsgId")]
    pub msg_id: u64,
    #[serde(default)]
    #[serde(rename = "MsgDataId")]
    pub msg_data_id: String,
    #[serde(default)]
    #[serde(rename = "Idx")]
    pub idx: String,
}

/*
<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[fromUser]]></FromUserName>
  <CreateTime>1351776360</CreateTime>
  <MsgType><![CDATA[location]]></MsgType>
  <Location_X>23.134521</Location_X>
  <Location_Y>113.358803</Location_Y>
  <Scale>20</Scale>
  <Label><![CDATA[位置信息]]></Label>
  <MsgId>1234567890123456</MsgId>
   <MsgDataId>xxxx</MsgDataId>
  <Idx>xxxx</Idx>
</xml>
 */
#[derive(serde::Deserialize, Debug)]
pub struct LocationMsg {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "Location_X")]
    pub location_x: f64,
    #[serde(rename = "Location_Y")]
    pub location_y: f64,
    #[serde(rename = "Scale")]
    pub scale: u64,
    #[serde(rename = "Label")]
    pub label: String,
    #[serde(rename = "MsgId")]
    pub msg_id: u64,
    #[serde(default)]
    #[serde(rename = "MsgDataId")]
    pub msg_data_id: String,
    #[serde(default)]
    #[serde(rename = "Idx")]
    pub idx: String,
}

/*
<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[fromUser]]></FromUserName>
  <CreateTime>1351776360</CreateTime>
  <MsgType><![CDATA[link]]></MsgType>
  <Title><![CDATA[公众平台官网链接]]></Title>
  <Description><![CDATA[公众平台官网链接]]></Description>
  <Url><![CDATA[url]]></Url>
  <MsgId>1234567890123456</MsgId>
   <MsgDataId>xxxx</MsgDataId>
  <Idx>xxxx</Idx>
</xml>
 */
#[derive(serde::Deserialize, Debug)]
pub struct LinkMsg {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Url")]
    pub url: String,
    #[serde(rename = "MsgId")]
    pub msg_id: u64,
    #[serde(default)]
    #[serde(rename = "MsgDataId")]
    pub msg_data_id: String,
    #[serde(default)]
    #[serde(rename = "Idx")]
    pub idx: String,
}

/*
关注/取消关注事件
<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[FromUser]]></FromUserName>
  <CreateTime>123456789</CreateTime>
  <MsgType><![CDATA[event]]></MsgType>
  <Event><![CDATA[subscribe]]></Event>
</xml>
 */
#[derive(serde::Deserialize, Debug)]
pub struct SubscribeEvent {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "Event")]
    pub event: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct UnSubscribeEvent {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "Event")]
    pub event: String,
}

/*
扫描带参数二维码事件
用户扫描带场景值二维码时，可能推送以下两种事件：

如果用户还未关注公众号，则用户可以关注公众号，关注后微信会将带场景值关注事件推送给开发者。
如果用户已经关注公众号，则微信会将带场景值扫描事件推送给开发者。
1. 用户未关注时，进行关注后的事件推送

推送XML数据包示例：

<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[FromUser]]></FromUserName>
  <CreateTime>123456789</CreateTime>
  <MsgType><![CDATA[event]]></MsgType>
  <Event><![CDATA[subscribe]]></Event>
  <EventKey><![CDATA[qrscene_123123]]></EventKey>
  <Ticket><![CDATA[TICKET]]></Ticket>
</xml>
 */
#[derive(serde::Deserialize, Debug)]
pub struct SubscribeWithSceneEvent {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "EventKey")]
    pub event_key: String,
    #[serde(rename = "Ticket")]
    pub ticket: String,
}

/*
2. 用户已关注时的事件推送

推送XML数据包示例：

<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[FromUser]]></FromUserName>
  <CreateTime>123456789</CreateTime>
  <MsgType><![CDATA[event]]></MsgType>
  <Event><![CDATA[SCAN]]></Event>
  <EventKey><![CDATA[SCENE_VALUE]]></EventKey>
  <Ticket><![CDATA[TICKET]]></Ticket>
</xml>
 */
#[derive(serde::Deserialize, Debug)]
pub struct ScanEvent {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "EventKey")]
    pub event_key: String,
    #[serde(rename = "Ticket")]
    pub ticket: String,
}

/*
<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[fromUser]]></FromUserName>
  <CreateTime>123456789</CreateTime>
  <MsgType><![CDATA[event]]></MsgType>
  <Event><![CDATA[LOCATION]]></Event>
  <Latitude>23.137466</Latitude>
  <Longitude>113.352425</Longitude>
  <Precision>119.385040</Precision>
</xml>
 */
#[derive(serde::Deserialize, Debug)]
pub struct LocationEvent {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "Latitude")]
    pub latitude: f64,
    #[serde(rename = "Longitude")]
    pub longitude: f64,
    #[serde(rename = "Precision")]
    pub precision: f64,
}

/*自定义菜单事件
用户点击自定义菜单后，微信会把点击事件推送给开发者，请注意，点击菜单弹出子菜单，不会产生上报。

点击菜单拉取消息时的事件推送

推送XML数据包示例：

<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[FromUser]]></FromUserName>
  <CreateTime>123456789</CreateTime>
  <MsgType><![CDATA[event]]></MsgType>
  <Event><![CDATA[CLICK]]></Event>
  <EventKey><![CDATA[EVENTKEY]]></EventKey>
</xml>
 */
#[derive(serde::Deserialize, Debug)]
pub struct ClickEvent {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "EventKey")]
    pub event_key: String,
}

/*
点击菜单跳转链接时的事件推送

推送XML数据包示例：

<xml>
  <ToUserName><![CDATA[toUser]]></ToUserName>
  <FromUserName><![CDATA[FromUser]]></FromUserName>
  <CreateTime>123456789</CreateTime>
  <MsgType><![CDATA[event]]></MsgType>
  <Event><![CDATA[VIEW]]></Event>
  <EventKey><![CDATA[www.qq.com]]></EventKey>
</xml>
 */
#[derive(serde::Deserialize, Debug)]
pub struct ViewEvent {
    #[serde(rename = "ToUserName")]
    pub to_user_name: String,
    #[serde(rename = "FromUserName")]
    pub from_user_name: String,
    #[serde(rename = "CreateTime")]
    pub create_time: u64,
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(rename = "Event")]
    pub event: String,
    #[serde(rename = "EventKey")]
    pub event_key: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct BaseMsg {
    #[serde(rename = "MsgType")]
    pub msg_type: String,
    #[serde(default)]
    #[serde(rename = "Event")]
    pub event: String,
}

// 解析消息，判断消息类型，然后根据消息类型进行处理
#[derive(Debug)]
pub enum Msg {
    TextMsg(TextMsg),
    ImageMsg(ImageMsg),
    VoiceMsg(VoiceMsg),
    VideoMsg(VideoMsg),
    ShortVideoMsg(ShortVideoMsg),
    LocationMsg(LocationMsg),
    LinkMsg(LinkMsg),
    SubscribeEvent(SubscribeEvent),
    UnSubscribeEvent(UnSubscribeEvent),
    SubscribeWithSceneEvent(SubscribeWithSceneEvent),
    ScanEvent(ScanEvent),
    LocationEvent(LocationEvent),
    ClickEvent(ClickEvent),
    ViewEvent(ViewEvent),
}

impl Msg {
    pub fn from_str(data: &str) -> Result<Msg, String> {
        let msg_data = match from_str::<BaseMsg>(data) {
            Ok(data) => data,
            Err(e) => {
                return Err(format!("解析数据失败: {}", e));
            }
        };
        let msg_type = msg_data.msg_type.as_str();
        match msg_type {
            "text" => {
                let msg_data: TextMsg = match from_str(data) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(format!("解析数据失败: {}", e));
                    }
                };
                Ok(Msg::TextMsg(msg_data))
            }
            "image" => {
                let msg_data: ImageMsg = match from_str(data) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(format!("解析数据失败: {}", e));
                    }
                };
                Ok(Msg::ImageMsg(msg_data))
            }
            "voice" => {
                let msg_data: VoiceMsg = match from_str(data) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(format!("解析数据失败: {}", e));
                    }
                };
                Ok(Msg::VoiceMsg(msg_data))
            }
            "video" => {
                let msg_data: VideoMsg = match from_str(data) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(format!("解析数据失败: {}", e));
                    }
                };
                Ok(Msg::VideoMsg(msg_data))
            }
            "shortvideo" => {
                let msg_data: ShortVideoMsg = match from_str(data) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(format!("解析数据失败: {}", e));
                    }
                };
                Ok(Msg::ShortVideoMsg(msg_data))
            }
            "location" => {
                let msg_data: LocationMsg = match from_str(data) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(format!("解析数据失败: {}", e));
                    }
                };
                Ok(Msg::LocationMsg(msg_data))
            }
            "link" => {
                let msg_data: LinkMsg = match from_str(data) {
                    Ok(data) => data,
                    Err(e) => {
                        return Err(format!("解析数据失败: {}", e));
                    }
                };
                Ok(Msg::LinkMsg(msg_data))
            }
            "event" => {
                let event = msg_data.event.as_str();
                match event {
                    "subscribe" => {
                        let msg_data: SubscribeEvent = match from_str(data) {
                            Ok(data) => data,
                            Err(e) => {
                                return Err(format!("解析数据失败: {}", e));
                            }
                        };
                        Ok(Msg::SubscribeEvent(msg_data))
                    }
                    "unsubscribe" => {
                        let msg_data: UnSubscribeEvent = match from_str(data) {
                            Ok(data) => data,
                            Err(e) => {
                                return Err(format!("解析数据失败: {}", e));
                            }
                        };
                        Ok(Msg::UnSubscribeEvent(msg_data))
                    }
                    "SCAN" => {
                        let msg_data: ScanEvent = match from_str(data) {
                            Ok(data) => data,
                            Err(e) => {
                                return Err(format!("解析数据失败: {}", e));
                            }
                        };
                        Ok(Msg::ScanEvent(msg_data))
                    }
                    "LOCATION" => {
                        let msg_data: LocationEvent = match from_str(data) {
                            Ok(data) => data,
                            Err(e) => {
                                return Err(format!("解析数据失败: {}", e));
                            }
                        };
                        Ok(Msg::LocationEvent(msg_data))
                    }
                    "CLICK" => {
                        let msg_data: ClickEvent = match from_str(data) {
                            Ok(data) => data,
                            Err(e) => {
                                return Err(format!("解析数据失败: {}", e));
                            }
                        };
                        Ok(Msg::ClickEvent(msg_data))
                    }
                    "VIEW" => {
                        let msg_data: ViewEvent = match from_str(data) {
                            Ok(data) => data,
                            Err(e) => {
                                return Err(format!("解析数据失败: {}", e));
                            }
                        };
                        Ok(Msg::ViewEvent(msg_data))
                    }
                    _ => Err("未知事件类型".to_string()),
                }
            }
            _ => Err("未知消息类型".to_string()),
        }
    }
}

impl FromStr for Msg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Msg::from_str(s)
    }
}

impl TryFrom<String> for Msg {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Msg::from_str(value.as_str())
    }
}

