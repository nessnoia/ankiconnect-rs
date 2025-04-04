use ankiconnect_rs::builders::Query;
use ankiconnect_rs::{AnkiClient, Result};
use httpmock::prelude::*;
use serde_json::json;

// Helper function to create a mock AnkiClient connected to the given mock server
fn create_mock_client(server: &MockServer) -> AnkiClient {
    AnkiClient::with_connection(&server.host(), server.port())
}

#[test]
fn test_find_cards() -> Result<()> {
    // Arrange
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/").json_body(json!({
            "action": "findCards",
            "version": 6,
            "params": {
                "query": "deck:current"
            }
        }));

        then.status(200)
            .header("content-type", "application/json")
            .json_body(json!({
                "result": [1494723142483_u64, 1494703460437_u64, 1494703479525_u64],
                "error": null
            }));
    });

    let client = create_mock_client(&server);

    // Act
    let cards = client
        .cards()
        .find(&Query::custom("deck:current".to_string()));

    // Assert
    mock.assert();

    let cards = cards?;
    assert_eq!(cards.len(), 3);
    assert_eq!(cards[0].value(), 1494723142483);
    assert_eq!(cards[1].value(), 1494703460437);
    assert_eq!(cards[2].value(), 1494703479525);

    Ok(())
}

// #[test]
// fn test_get_ease_factors() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "getEaseFactors",
//                 "version": 6,
//                 "params": {
//                     "cards": [1483959291685, 1483959293217]
//                 }
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": [4100, 3900],
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let ease_factors = client.cards().get_ease_factors(&[CardId(1483959291685), CardId(1483959293217)]);
//
//     // Assert
//     mock.assert();
//
//     let ease_factors = ease_factors?;
//     assert_eq!(ease_factors.len(), 2);
//     assert_eq!(ease_factors[0], 4100);
//     assert_eq!(ease_factors[1], 3900);
//
//     Ok(())
// }

// #[test]
// fn test_set_ease_factors() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "setEaseFactors",
//                 "version": 6,
//                 "params": {
//                     "cards": [1483959291685, 1483959293217],
//                     "easeFactors": [4100, 3900]
//                 }
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": [true, true],
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let results = client.cards().set_ease_factors(
//         &[CardId(1483959291685), CardId(1483959293217)],
//         &[4100, 3900]
//     );
//
//     // Assert
//     mock.assert();
//
//     let results = results?;
//     assert_eq!(results.len(), 2);
//     assert!(results[0]);
//     assert!(results[1]);
//
//     Ok(())
// }

// #[test]
// fn test_suspend_cards() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "suspend",
//                 "version": 6,
//                 "params": {
//                     "cards": [1483959291685_u64, 1483959293217_u64]
//                 }
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": true,
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let result = client.cards().suspend_cards(&[CardId(1483959291685), CardId(1483959293217)]);
//
//     // Assert
//     mock.assert();
//     assert!(result.is_ok());
//
//     Ok(())
// }

// #[test]
// fn test_unsuspend_cards() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "unsuspend",
//                 "version": 6,
//                 "params": {
//                     "cards": [1483959291685_u64, 1483959293217_u64]
//                 }
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": true,
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let result = client.cards().unsuspend_cards(&[CardId(1483959291685), CardId(1483959293217)]);
//
//     // Assert
//     mock.assert();
//     assert!(result.is_ok());
//
//     Ok(())
// }

// #[test]
// fn test_are_cards_suspended() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "areSuspended",
//                 "version": 6,
//                 "params": {
//                     "cards": [1483959291685, 1483959293217, 1234567891234]
//                 }
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": [false, true, null],
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let results = client.cards().are_suspended(&[
//         CardId(1483959291685),
//         CardId(1483959293217),
//         CardId(1234567891234)
//     ]);
//
//     // Assert
//     mock.assert();
//
//     let results = results?;
//     assert_eq!(results.len(), 3);
//     assert_eq!(results[0], Some(false));
//     assert_eq!(results[1], Some(true));
//     assert_eq!(results[2], None);
//
//     Ok(())
// }

// #[test]
// fn test_get_intervals() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "getIntervals",
//                 "version": 6,
//                 "params": {
//                     "cards": [1502298033753, 1502298036657],
//                     "complete": true
//                 }
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": [
//                     [-120, -180, -240, -300, -360, -14400],
//                     [-120, -180, -240, -300, -360, -14400, 1, 3]
//                 ],
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let intervals = client.cards().get_intervals(
//         &[CardId(1502298033753), CardId(1502298036657)],
//         true
//     );
//
//     // Assert
//     mock.assert();
//
//     let intervals = intervals?;
//     assert_eq!(intervals.len(), 2);
//     assert_eq!(intervals[0], vec![-120, -180, -240, -300, -360, -14400]);
//     assert_eq!(intervals[1], vec![-120, -180, -240, -300, -360, -14400, 1, 3]);
//
//     Ok(())
// }

// #[test]
// fn test_cards_info() -> Result<()> {
//     // Arrange
//     let server = MockServer::start();
//
//     let mock = server.mock(|when, then| {
//         when.method(POST)
//             .path("/")
//             .json_body(json!({
//                 "action": "cardsInfo",
//                 "version": 6,
//                 "params": {
//                     "cards": [1498938915662, 1502098034048]
//                 }
//             }));
//
//         then.status(200)
//             .header("content-type", "application/json")
//             .json_body(json!({
//                 "result": [
//                     {
//                         "answer": "back content",
//                         "question": "front content",
//                         "deckName": "Default",
//                         "modelName": "Basic",
//                         "fieldOrder": 1,
//                         "fields": {
//                             "Front": {"value": "front content", "order": 0},
//                             "Back": {"value": "back content", "order": 1}
//                         },
//                         "css": "p {font-family:Arial;}",
//                         "cardId": 1498938915662,
//                         "interval": 16,
//                         "note": 1502298033753,
//                         "ord": 1,
//                         "type": 0,
//                         "queue": 0,
//                         "due": 1,
//                         "reps": 1,
//                         "lapses": 0,
//                         "left": 6,
//                         "mod": 1629454092
//                     },
//                     {
//                         "answer": "back content",
//                         "question": "front content",
//                         "deckName": "Default",
//                         "modelName": "Basic",
//                         "fieldOrder": 0,
//                         "fields": {
//                             "Front": {"value": "front content", "order": 0},
//                             "Back": {"value": "back content", "order": 1}
//                         },
//                         "css": "p {font-family:Arial;}",
//                         "cardId": 1502098034048,
//                         "interval": 23,
//                         "note": 1502298033753,
//                         "ord": 1,
//                         "type": 0,
//                         "queue": 0,
//                         "due": 1,
//                         "reps": 1,
//                         "lapses": 0,
//                         "left": 6
//                     }
//                 ],
//                 "error": null
//             }));
//     });
//
//     let client = create_mock_client(&server);
//
//     // Act
//     let cards_info = client.cards().get_cards_info(&[CardId(1498938915662), CardId(1502098034048)]);
//
//     // Assert
//     mock.assert();
//
//     let cards_info = cards_info?;
//     assert_eq!(cards_info.len(), 2);
//     assert_eq!(cards_info[0].card_id.value(), 1498938915662);
//     assert_eq!(cards_info[0].deck_name, "Default");
//     assert_eq!(cards_info[0].model_name, "Basic");
//     assert_eq!(cards_info[0].question, "front content");
//     assert_eq!(cards_info[0].answer, "back content");
//     assert_eq!(cards_info[0].interval, 16);
//
//     assert_eq!(cards_info[1].card_id.value(), 1502098034048);
//     assert_eq!(cards_info[1].deck_name, "Default");
//     assert_eq!(cards_info[1].model_name, "Basic");
//     assert_eq!(cards_info[1].question, "front content");
//     assert_eq!(cards_info[1].answer, "back content");
//     assert_eq!(cards_info[1].interval, 23);
//
//     Ok(())
// }
