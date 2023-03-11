use crate::views::message::view::MessageRequest;

pub fn validate(request: &MessageRequest) -> Result<(), String> {
    if request.message.text().is_none() {
        return Err(String::from("You should place some text after command"));
    }

    let text: &str = request.message.text().unwrap();
    let mut lines = text
        .split("\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if lines.len() != 1 {
        return Err(String::from(
            "Only oneline votes are supported for 'point story' template",
        ));
    }

    let task_line = lines.first().unwrap();
    let parts = task_line
        .split(" ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if parts.len() <= 1 {
        return Err(String::from("Task not placed"));
    }

    Ok(())
}
