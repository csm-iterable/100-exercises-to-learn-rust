// TODO: Add &mut-setters to the `Ticket` struct for each of its fields.
//   Make sure to enforce the same validation rules you have in `Ticket::new`!
//   Even better, extract that logic and reuse it in both places. You can use
//   private functions or private static methods for that.

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        Self::perform_validation(Some(&title), Some(&description), Some(&status));

        Ticket {
            title,
            description,
            status,
        }
    }

    fn perform_validation(
        title: Option<&String>,
        description: Option<&String>,
        status: Option<&String>,
    ) {
        match title {
            None => (),
            Some(title) => {
                if title.len() > 50 {
                    panic!("Title cannot be longer than 50 bytes");
                }

                if title.is_empty() {
                    panic!("Title cannot be empty");
                }
            }
        }

        match description {
            None => (),
            Some(description) => {
                if description.len() > 500 {
                    panic!("Description cannot be longer than 500 bytes");
                }

                if description.is_empty() {
                    panic!("Description cannot be empty")
                }
            }
        }

        match status {
            None => (),
            Some(status) => {
                if status != "To-Do" && status != "In Progress" && status != "Done" {
                    panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
                }
            }
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn set_title(&mut self, new_title: String) {
        Self::perform_validation(Some(&new_title), None, None);
        self.title = new_title;
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn set_description(&mut self, new_description: String) {
        Self::perform_validation(None, Some(&new_description), None);
        self.description = new_description;
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    pub fn set_status(&mut self, new_status: String) {
        Self::perform_validation(None, None, Some(&new_status));
        self.status = new_status;
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        let mut ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        ticket.set_title("A new title".into());
        ticket.set_description("A new description".into());
        ticket.set_status("Done".into());

        assert_eq!(ticket.title(), "A new title");
        assert_eq!(ticket.description(), "A new description");
        assert_eq!(ticket.status(), "Done");
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_title("".into());
    }

    #[test]
    #[should_panic(expected = "Description cannot be empty")]
    fn description_cannot_be_empty() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_description("".into());
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
