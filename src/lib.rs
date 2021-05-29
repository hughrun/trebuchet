pub mod utils {

  use std::iter;
  use rand::{Rng, distributions::Alphanumeric, thread_rng};

// Structs and enums
// =================

  pub struct User {
    email: String,
    capsule: String,
    token: String
  }

  struct Token {
    token: String,
    email: String,
    expiry: String,
    used: bool
  }

  #[derive(Debug)]
  pub enum EmailType {
    Confirm,
    LogIn
  }

  enum TrebuchetError {
    TokenError(String),
    EmailError(String)
  }

// Orphaned Functions
// =================

  pub fn build_user(email: &str, capsule: &str) -> User {
    User {
      email: String::from(email),
      capsule: String::from(capsule),
      token: create_otp()
    }
  }

  fn create_otp() -> String {
    let mut rng = thread_rng();
    let chars: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(52)
        .collect();
    return chars
  }

// Implementations
// ================

  impl User {

    // PUBLIC FUNCTIONS
    // ----------------

    pub fn add_user(self) {

      // add user to database
    
      // send email to user
      self.initiate_login(EmailType::Confirm)
    }

    pub fn initiate_login(self, etype: EmailType) {

      // find user in DB
      // set self.capsule value

      // create expiry date
      // add token to DB - token, email, expiry

      // send email
      self.send_email(etype)
    }

    // PRIVATE FUNCTIONS
    // ----------------

    fn send_email(self, email_type: EmailType) {
      // create URL
      let root_domain = "https://example.com"; // TODO: this needs to be an ENV value
      let email = urlencoding::encode(&self.email);
      let link = format!("{}/{:?}?token={}&email={}", root_domain, email_type, self.token, email);

      // email text templates
      let confirmation_email_text = format!("Hello!\n\nYou are being invited to publish a Gemini capsule named {}, via {}.\n\nOpen the link below to confirm.\n\n<a href=\"{}\">{}</a>", self.capsule, root_domain, link, link);

      let login_email_text = format!("Hello!\n\nYou or someone else initiated a login at {}.\n\nOpen the link below to complete your login.\n\n<a href=\"{}\">{}</a>\n\nIf this was not you, ignore this email or advise your server administrator.", root_domain, link, link);

      // send email according to email_type
      //TESTING
      match email_type {
        EmailType::Confirm => println!("{}", confirmation_email_text),
        EmailType::LogIn => println!("{}", login_email_text)
      }

    }
    
    fn match_token(self) -> Result<Self,TrebuchetError>{
          // if no match look in expired_tokens table
      // if in expired_tokens check if used == true
      // if true, return TrebuchetError::TokenError("Token already used".to_string())
      // if false return TrebuchetError::TokenError("Token has expired".to_string())
      // if no match anywhere return TrebuchetError::TokenError("Token not recognised".to_string())
      // if both ok check email matches
      // if no match return TrebuchetError::EmailError("Email address not found")
      
      // find token in DB
      // TESTING
      let token_matches = false;
      match token_matches {
        true => Ok(self),
        false => {
          // TESTING
          let token = Token {
            token: "123abc".to_string(),
            email: "hugh@test.tld".to_string(),
            expiry: "1622247491".to_string(),
            used: false
          };
          // TODO: replace this with a check of the DB that returns a Token if something is found and nothing if not
          let unmatched_token = Some(token);
          match unmatched_token {
            Some(token) => {
              match token.used {
                true => Err(TrebuchetError::TokenError("Token already used".to_string())),
                false => Err(TrebuchetError::TokenError("Token has expired".to_string()))
              }
            },
            None => Err(TrebuchetError::TokenError("Token not recognised".to_string()))
          }
        }
      }
    }
  }
}