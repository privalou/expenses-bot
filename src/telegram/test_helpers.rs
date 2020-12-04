use mockito::{mock, Matcher, Mock};
use serde_json::json;

use crate::telegram::types::Message;

pub fn mock_send_message_success(token: &str, message: &Message) -> Mock {
    let mock_text_message = match message.text.contains("\n") {
        true => "Successfully sent",
        false => message.text,
    };

    let request_body = format!(
        r#"
    {{
        "ok":true,
        "result":{{
            "message_id":691,
            "from":{{
                "id":414141,
                "is_bot":true,
                "first_name":"Bot",
                "username":"Bot"
            }},
            "chat":{{
                "id":123,
                "first_name":"Name",
                "username":"username",
                "type":"private"
            }},
            "date":1581200384,
            "text":"{}"
        }}
    }}"#,
        mock_text_message
    );
    mock("POST", format!("/bot{}/sendMessage", token).as_str())
        .match_body(Matcher::Json(json!(message)))
        .with_status(200)
        .with_body(request_body)
        .with_header("content-type", "application/json")
        .expect(1)
        .create()
}
