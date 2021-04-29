use super::bulk_array;
use super::Response;

pub struct BlocksInChainResponse {
    pub block_ids: bulk_array::BulkArray<String>,
}

impl Response for BlocksInChainResponse {
    type E = bulk_array::ParseError;

    /// Parses a response string in the form
    /// `"[["alpha_numeric_block_id_string"], ["..."]]"` into a
    /// [`BlocksInChainResponse`](Self).
    fn from_response_str(response: &str) -> Result<Self, Self::E> {
        let block_ids = bulk_array::BulkArray::from_str(response)?;

        Ok(Self { block_ids })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_blocks_in_chain_from_response_empty_ok() {
        let mock_response = "";

        let blocks_response = BlocksInChainResponse::from_response_str(mock_response);
        assert!(blocks_response.is_ok());

        let blocks = blocks_response.unwrap().block_ids.into_vec();
        assert!(blocks.is_empty());
    }

    #[test]
    fn get_blocks_in_chain_from_response_flat_array_fails() {
        let mock_response = r#"["blockId1"]"#;

        let blocks_response = BlocksInChainResponse::from_response_str(mock_response);
        assert!(blocks_response.is_err());
    }

    #[test]
    fn get_blocks_in_chain_from_empty_list_ok() {
        let mock_response = "[]";
        let blocks_response = BlocksInChainResponse::from_response_str(mock_response);
        assert!(blocks_response.is_ok());

        let blocks = blocks_response.unwrap().block_ids.into_vec();
        assert!(blocks.is_empty());
    }

    #[test]
    #[should_panic]
    fn get_blocks_in_chain_from_malformed_response_fails() {
        let mock_response = "[[]]";

        let blocks_response = BlocksInChainResponse::from_response_str(mock_response);
        assert!(blocks_response.is_err());
    }

    #[test]
    fn get_blocks_in_chain_from_response_single_ok() {
        let mock_block_id = "blockId1";
        let mock_response = format!(r#"[["{}"]]"#, mock_block_id);

        let blocks_response = BlocksInChainResponse::from_response_str(&mock_response);
        assert!(blocks_response.is_ok());

        let mut blocks = blocks_response.unwrap().block_ids.into_vec();
        assert!(blocks.len() == 1);

        let parsed_block_id_result = blocks.pop();
        assert!(parsed_block_id_result.is_some());

        let parsed_block_id = parsed_block_id_result.unwrap();
        assert_eq!(&parsed_block_id, mock_block_id);
    }

    #[test]
    fn get_blocks_in_chain_from_response_multiple_ok() {
        let mock_block_ids = ["blockId1", "blockId2", "blockId3"];
        let mock_response = format!(
            "[{}]",
            mock_block_ids
                .iter()
                .map(|block_id| format!(r#"["{}"]"#, &block_id))
                .collect::<Vec<String>>()
                .join(",")
        );

        let blocks_response = BlocksInChainResponse::from_response_str(&mock_response);
        assert!(blocks_response.is_ok());

        let mut blocks = blocks_response.unwrap().block_ids.into_vec();
        assert!(blocks.len() == 3);

        for mock_block_id in mock_block_ids.iter().rev() {
            let block_id_to_compare = blocks.pop().unwrap();
            assert_eq!(&block_id_to_compare, mock_block_id);
        }
    }
}
