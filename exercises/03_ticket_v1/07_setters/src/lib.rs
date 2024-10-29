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
        Ticket::validate_title_not_empty(&title);
        Ticket::validate_title_length(&title);

        Ticket::validate_description_not_empty(&description);
        Ticket::validate_description_length(&description);

        Ticket::validate_status(&status);

        Ticket {
            title,
            description,
            status,
        }
    }

    fn validate_title_not_empty(title: &String) {
        match title {
            panic!("Title cannot be empty")
        }
    }

    fn validate_title_length(title: &String) {
        if title.len() > 50 {
            panic!("Title cannot be longer than 50 bytes")
                }
            }
        }
    }

    fn validate_description_not_empty(description: &String) {
            None => (),
            Some(description) => {
                if description.len() > 500 {
                    panic!("Description cannot be longer than 500 bytes");
        }
    }

    fn validate_description_length(description: &String) {
                if description.is_empty() {
                    panic!("Description cannot be empty")
                }
            }
        }
    }

    fn validate_status(status: &String) {
            None => (),
            Some(status) => {
                if status != "To-Do" && status != "In Progress" && status != "Done" {
                    panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    // pub fn set_title(&mut self, title: String) {
    //     Ticket::validate_title_not_empty(&title);
    //     Ticket::validate_title_length(&title);

    //     self.title = title;
    // }
    pub fn set_title(mut self, new_title: String) -> Self {
        Ticket::validate_title_not_empty(&new_title);
        Ticket::validate_title_length(&new_title);

        self.title = new_title;
        self
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    // pub fn set_description(&mut self, description: String) {
    //     Ticket::validate_description_length(&description);
    //     Ticket::validate_description_not_empty(&description);

    //     self.description = description
    // }
    pub fn set_description(mut self, new_description: String) -> Self {
        Ticket::validate_description_length(&new_description);
        Ticket::validate_description_not_empty(&new_description);

        self.description = new_description;
        self
    }

    pub fn status(&self) -> &String {
        &self.status
    }

    // pub fn set_status(&mut self, status: String) {
    //     Ticket::validate_status(&status);
    //     self.status = status;
    // }
    pub fn set_status(mut self, new_status: String) -> Self {
        Ticket::validate_status(&new_status);
        self.status = new_status;

        self
    }
}

// ! the implementation of these tests requires that we use a specific pattern for setters
#[cfg(test)]
mod tests {
    use super::Ticket;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    fn works() {
        // if we use the mut self pattern, we need to use a variable to keep ownership
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        //we could do this all in one line
        let ticket = ticket
            .set_title("A new title".into())
            .set_description("A new description".into())
            .set_status("Done".into());

        // if we use the &mut self we need call everything on a separate line, because we don't take ownership so we don't return anything
        // ticket.set_title("A new title".into());
        // ticket.set_description("A new description".into());
        // ticket.set_status("Done".into());

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
        // if we use the return self pattern, we still need a variable
        // if we used the &mut self here, we wouldn't need a variable because we dont' return anything
        let _ = Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_title(overly_long_title());
    }

    #[test]
    #[should_panic(expected = "Description cannot be longer than 500 bytes")]
    fn description_cannot_be_longer_than_500_chars() {
        // if we use the return self pattern, we still need a variable
        // if we used the &mut self here, we wouldn't need a variable because we dont' return anything
        let _ = Ticket::new(valid_title(), valid_description(), "To-Do".into())
            .set_description(overly_long_description());
    }

    #[test]
    #[should_panic(expected = "Only `To-Do`, `In Progress`, and `Done` statuses are allowed")]
    fn status_must_be_valid() {
        Ticket::new(valid_title(), valid_description(), "To-Do".into()).set_status("Funny".into());
    }
}
