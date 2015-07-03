use std::io;
use std::io::Read;
use hyper;
use hyper::Client;
use hyper::header::Connection;
use rustc_serialize::json::{ParserError, Json};

pub struct Game {
    pub score: u64,
    current_question: Option<Question>
}

impl Game {
    pub fn new() -> Game {
        Game{score: 0, current_question: None}
    }

    pub fn new_question(&mut self) -> Result<Question, GameError> {
        let json_question = try!(self.get_question_json());
        let question = try!(Question::from_json(json_question));
        self.current_question = Some(question.clone());
        Ok(question)
    }

    fn get_question_json(&self) -> Result<Json, GameError> {
        let client = Client::new();
        let mut res = try!(client.get("http://jservice.io/api/random")
                           .header(Connection::close())
                           .send());

        let mut body = String::new();
        try!(res.read_to_string(&mut body));

        Ok(try!(Json::from_str(&body.trim())))
    }

    pub fn verify_question_answer(&self, answer: &str) -> Result<bool, GameError> {
        match self.current_question {
            Some(ref question) => {
                let result = question.title.eq(answer);
                Ok(result)
            },
            None => {
                Err(GameError::NoCurrentQuestion)
            }
        }
    }
}
#[derive (Clone, Debug, PartialEq)]
struct Question {
    pub title: String,
    pub answer: String,
    pub id: u64,
    pub value: u64
}

impl Question {
    fn from_json(json: Json) -> Result<Question, GameError> {

        let data = try!(json.as_array().ok_or("failed to parse jservice.io random question json, not an array"));
        let data = try!(data.get(0).ok_or("failed to parse jservice.io random question json, not an array"));
        let data = try!(data.as_object().ok_or("failed to parse jservice.io random question json, not an object"));

        let title = try!(data.get("question")
                         .ok_or("failed to parse jservice.io random question json field question"));
        let title = try!(title.as_string().
                         ok_or("failed to parse jservice.io random question json field question"))
            .to_string();

        let answer = try!(data.get("answer")
                          .ok_or("failed to parse jservice.io random question json field answer"));

        let answer = try!(answer.as_string().
                          ok_or("failed to parse jservice.io random question json field answer"))
            .to_string();

        let id = try!(data.get("id")
                      .ok_or("failed to parse jservice.io random question json field id"));
        let id= try!(id.as_u64().
                     ok_or("failed to parse jservice.io random question json field id"));
        let value = try!(data.get("value")
                         .ok_or("failed to parse jservice.io random question json field value"));
        let value= try!(value.as_u64().
                        ok_or("failed to parse jservice.io random question json field value"));

        Ok(Question{
            title: title,
            answer: answer,
            id: id,
            value: value
        })
    }
}


#[derive(Debug)]
pub enum GameError {
    HyperError(hyper::error::Error),
    IoError(io::Error),
    JsonParserError(ParserError),
    QuestionParserError(&'static str),
    NoCurrentQuestion
}

impl From<hyper::error::Error> for GameError {

    fn from(err: hyper::error::Error) -> GameError {
        GameError::HyperError(err)
    }
}

impl From<io::Error> for GameError {

    fn from(err: io::Error) -> GameError {
        GameError::IoError(err)
    }
}

impl From<ParserError> for GameError {

    fn from(err: ParserError) -> GameError {
        GameError::JsonParserError(err)
    }
}

impl From<&'static str> for GameError {

    fn from(err: &'static str) -> GameError {
        GameError::QuestionParserError(err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Question;
    use std::io;
    use std::io::Read;
    use hyper;
    use hyper::Client;
    use hyper::header::Connection;
    use rustc_serialize::json::{ParserError, Json};

    #[test]
    fn test_can_generate_new_question() {
        let mut game = Game::new();
        let question = game.new_question();
        question.unwrap();
    }


    #[test]
    fn test_can_parse_json() {
        let mut game = Game::new();
        let json_string: &str = r#"[{"id":95779,"answer":"1","question":"An NFL game ended 17-7; the losing team scored no touchdowns but this many field goals","value":600,"airdate":"2010-06-03T12:00:00.000Z","created_at":"2014-02-14T02:01:25.720Z","updated_at":"2014-02-14T02:01:25.720Z","category_id":12744,"game_id":null,"invalid_count":null,"category":{"id":12744,"title":"let's be logical","created_at":"2014-02-14T02:01:25.025Z","updated_at":"2014-02-14T02:01:25.025Z","clues_count":5}}]"#;
        let json = Json::from_str(json_string).unwrap();
        let question = Question::from_json(json).unwrap();
        let question2 = Question{
            id: 95779,
            title: "An NFL game ended 17-7; the losing team scored no touchdowns but this many field goals".to_string(),
            answer: "1".to_string(),
            value: 600
        };
        assert_eq!(question, question2);
    }

    #[test]
    fn expect_array_json() {
        let mut game = Game::new();
        let json_string: &str = r#"{"id":95779,"answer":"1","question":"An NFL game ended 17-7; the losing team scored no touchdowns but this many field goals","value":600,"airdate":"2010-06-03T12:00:00.000Z","created_at":"2014-02-14T02:01:25.720Z","updated_at":"2014-02-14T02:01:25.720Z","category_id":12744,"game_id":null,"invalid_count":null,"category":{"id":12744,"title":"let's be logical","created_at":"2014-02-14T02:01:25.025Z","updated_at":"2014-02-14T02:01:25.025Z","clues_count":5}}"#;
        let json = Json::from_str(json_string).unwrap();
        let question = Question::from_json(json);
        match question {
            Err(GameError::QuestionParserError(msg)) => assert_eq!(msg, "failed to parse jservice.io random question json, not an array"),
            _ => assert!(false)
        }
    }

    #[test]
    fn expect_object_json() {
        "failed to parse jservice.io random question json, not an object";
        let mut game = Game::new();
        let json_string: &str = r#"[[{"id":95779,"answer":"1","question":"An NFL game ended 17-7; the losing team scored no touchdowns but this many field goals","value":600,"airdate":"2010-06-03T12:00:00.000Z","created_at":"2014-02-14T02:01:25.720Z","updated_at":"2014-02-14T02:01:25.720Z","category_id":12744,"game_id":null,"invalid_count":null,"category":{"id":12744,"title":"let's be logical","created_at":"2014-02-14T02:01:25.025Z","updated_at":"2014-02-14T02:01:25.025Z","clues_count":5}}]]"#;
        let json = Json::from_str(json_string).unwrap();
        let question = Question::from_json(json);
        match question {
            Err(GameError::QuestionParserError(msg)) => assert_eq!(msg, "failed to parse jservice.io random question json, not an object"),
            _ => assert!(false)
        }


    }
}
