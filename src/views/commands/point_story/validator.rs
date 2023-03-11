use crate::views::message::view::MessageRequest;

pub fn validate(request: &MessageRequest) -> Result<(), String> {
    if request.message.text().is_none() {
        return Err(String::from("You should place some text after command"));
    }

    let text: &str = request.message.text().unwrap();
    let mut lines = text.split("\n");
    if lines.by_ref().count() != 1 {
        return Err(String::from(
            "Only oneline votes are supported for 'point story' template",
        ));
    }

    let mut lines = text.split("\n");
    let task_line = lines.next().unwrap();
    if task_line.split(" ").by_ref().count() <= 1 {
        return Err(String::from("Task not placed"));
    }

    Ok(())
}
