pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        // ? the 'size' of a heap allocated object does _not_ contain the size of the actual stuff on the heap.
        // ? it contains the value of the record on the stack that tracks the heap allocated object
        // ? in the case of a string, it's the pointer to the heap value, the current length of the string, and the allocated capacity
        // ? each of the values in the stack frame are of type usize, hence size_of::<usize>() * 3
        assert_eq!(size_of::<String>(), size_of::<usize>() * 3);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Data layout" section of the Rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.
        // ? In this case, since the struct consists of 3 strings
        // ? and each struct member has size 3 (as described above)
        // ? we just multiply size_of::<usize>() by 9 (which is 3 * 3)
        assert_eq!(size_of::<Ticket>(), size_of::<usize>() * 9);
    }
}
