#[cfg(test)]
mod tests {
    use scupt_util::logger::logger_setup;

    use crate::raft_message::RAFT;
    use crate::test_dtm::tests::{InputType, test_raft_gut};

    #[test]
    fn test_raft_input_from_json() {
        logger_setup("debug");
        test_raft_gut(
            InputType::FromJsonFile("raft_trace.json".to_string()),
            3000, 3,
            RAFT.to_string(),
            None)
    }
}