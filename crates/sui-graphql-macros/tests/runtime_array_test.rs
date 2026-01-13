use sui_graphql_macros::QueryResponse;

#[test]
fn test_single_array_iteration() {
    #[derive(QueryResponse)]
    struct NodeNames {
        #[field(path = "nodes[].name")]
        names: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "nodes": [
            { "name": "alice" },
            { "name": "bob" },
            { "name": "charlie" }
        ]
    });
    let data = NodeNames::from_value(json).unwrap();
    assert_eq!(data.names, Some(vec!["alice".to_string(), "bob".to_string(), "charlie".to_string()]));
}

#[test]
fn test_nested_array_iteration() {
    #[derive(QueryResponse)]
    struct NestedData {
        #[field(path = "nodes[].edges[].id")]
        edge_ids: Option<Vec<Option<Vec<u64>>>>,
    }

    let json = serde_json::json!({
        "nodes": [
            { "edges": [{ "id": 1 }, { "id": 2 }] },
            { "edges": [{ "id": 3 }] },
            { "edges": [] }
        ]
    });
    let data = NestedData::from_value(json).unwrap();
    assert_eq!(data.edge_ids, Some(vec![Some(vec![1, 2]), Some(vec![3]), Some(vec![])]));
}

#[test]
fn test_array_at_end_of_path() {
    #[derive(QueryResponse)]
    struct ItemList {
        #[field(path = "items[]")]
        items: Option<Vec<u64>>,
    }

    let json = serde_json::json!({
        "items": [10, 20, 30]
    });
    let data = ItemList::from_value(json).unwrap();
    assert_eq!(data.items, Some(vec![10, 20, 30]));
}

#[test]
fn test_array_with_prefix_path() {
    #[derive(QueryResponse)]
    struct DeepArray {
        #[field(path = "data.response.nodes[].value")]
        values: Option<Vec<i32>>,
    }

    let json = serde_json::json!({
        "data": {
            "response": {
                "nodes": [
                    { "value": 100 },
                    { "value": 200 }
                ]
            }
        }
    });
    let data = DeepArray::from_value(json).unwrap();
    assert_eq!(data.values, Some(vec![100, 200]));
}

#[test]
fn test_empty_array() {
    #[derive(QueryResponse)]
    struct NodeNames {
        #[field(path = "nodes[].name")]
        names: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "nodes": []
    });
    let data = NodeNames::from_value(json).unwrap();
    assert_eq!(data.names, Some(vec![]));
}

#[test]
fn test_null_array_returns_none() {
    #[derive(QueryResponse)]
    struct NodeNames {
        #[field(path = "nodes[].name")]
        names: Option<Vec<String>>,
    }

    let json = serde_json::json!({
        "nodes": null
    });
    let data = NodeNames::from_value(json).unwrap();
    assert_eq!(data.names, None);
}

#[test]
fn test_null_nested_array() {
    #[derive(QueryResponse)]
    struct NestedData {
        #[field(path = "data.nodes[].name")]
        names: Option<Vec<String>>,
    }

    // When parent field (data) is null, the array should be None
    let json = serde_json::json!({
        "data": null
    });
    let data = NestedData::from_value(json).unwrap();
    assert_eq!(data.names, None);
}
