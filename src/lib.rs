#[allow(dead_code)]
pub mod utils {

  use std::{io, iter, fs::File};
  use rand::{Rng, distributions::Alphanumeric, thread_rng};
  use crate::database;

// Structs and enums
// =================

  pub struct User {
    pub email: String,
    pub capsule: String,
    pub token: String
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
    Delete,
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

  pub fn file_exists(path: &str) -> std::io::Result<()> {
    let _f = File::open(path)?;
    Ok(())
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

    pub fn add_user(self) -> Result<(), Box<(dyn std::error::Error)>>{

      // add user to database and send email
      database::add_user(self)?.initiate_login(EmailType::Confirm)?;
      Ok(())
    }

    pub fn confirm_user(self) -> Result<(), Box<(dyn std::error::Error)>>{

      // TODO: update token 
      
      // TODO: add user files

      // update database and send email
      database::confirm_user(self)?.build_email(EmailType::Confirm)?;

      //  TODO: probably should build in some error checking here to roll back if necessary
      Ok(())
    }

    pub fn delete_user(self) -> Result<(), Box<(dyn std::error::Error)>>{

      // TODO: delete user files
      // remove user from database and send email
      database::delete_user(self)?.build_email(EmailType::Delete)?;

      //  TODO: probably should build in some error checking here to roll back if necessary
      Ok(())
    }
    pub fn initiate_login(self, etype: EmailType) -> Result<(), io::Error> {
      // TODO:
      // find user in DB
      // set self.capsule value

      // create expiry date
      // add token to DB - token, email, expiry

      // send email
      self.build_email(etype)?;
      Ok(())
    }

    // PRIVATE FUNCTIONS
    // ----------------

    fn build_email(self, email_type: EmailType) -> Result<(), io::Error> {
      // create URL
      let root_domain = "https://example.com"; // TODO: this needs to be an ENV value
      let email = urlencoding::encode(&self.email);
      let link = format!("{}/{:?}?token={}&email={}", root_domain, email_type, self.token, email);

      // email text templates
      let confirmation_email_text = format!("Hello!\n\nYou are being invited to publish a Gemini capsule named {}, via {}.\n\nOpen the link below to confirm.\n\n<a href=\"{}\">{}</a>", self.capsule, root_domain, link, link);

      let deletion_email_text = format!("Hello\n\nYour Gemini capsule named {}, served from {}, has been deleted.", self.capsule, root_domain);

      let login_email_text = format!("Hello!\n\nYou or someone else initiated a login at {}.\n\nOpen the link below to complete your login.\n\n<a href=\"{}\">{}</a>\n\nIf this was not you, ignore this email or advise your server administrator.", root_domain, link, link);

      // send email according to email_type
      //TESTING
      match email_type {
        EmailType::Confirm => self.send_email(confirmation_email_text),
        EmailType::Delete => self.send_email(deletion_email_text),
        EmailType::LogIn => self.send_email(login_email_text)
      }

    }

    fn send_email(self, message: String) -> Result<(), io::Error> {
      // TODO:
      // TESTING:
      println!("{}", message);
      Ok(())
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

pub mod database {
  use crate::utils;
  use std::{io::prelude::*};
  use std::fs;
  use sqlite;
  
  pub fn build_tables() -> Result<(), sqlite::Error>{

    let connection = sqlite::Connection::open("trebuchet.db")?;

    connection.execute(
        "
        CREATE TABLE users (email TEXT UNIQUE, home_directory TEXT UNIQUE, confirmed INTEGER NOT NULL);
        CREATE TABLE tokens (token TEXT PRIMARY KEY, email TEXT NOT NULL, expiry TEXT NOT NULL);
        CREATE TABLE expired_tokens (token TEXT PRIMARY KEY, email TEXT NOT NULL, used INTEGER NOT NULL);
        CREATE TABLE cookies (user INTEGER NOT NULL, expiry TEXT NOT NULL);
        CREATE TABLE documents (owner INTEGER NOT NULL, content TEXT, title TEXT, tags TEXT, type TEXT NOT NULL, is_draft INTEGER, published_date TEXT DEFAULT CURRENT_TIMESTAMP, last_updated TEXT DEFAULT CURRENT_TIMESTAMP, uses_footer INTEGER, uses_header INTEGER);
        ",
    )?;
    Ok(())
  }

  pub fn create_default_files() -> std::io::Result<()>{

    // create directories
    fs::create_dir("./web")?;
    fs::create_dir("./capsules")?;
    println!("✔   default directories created");
    // create gemini index file
    let mut gem = fs::File::create("./capsules/index.gmi")?;
    gem.write_all(b"# Home\n\nWelcome to my site built with Trebuchet!\n\n=> gemini://trebuchet.hugh.run Find out more about Trebuchet")?;

    // create web files
    let mut html = fs::File::create("./web/index.html")?;
    html.write_all(b"<!DOCTYPE html>
    <html lang='en'>
    <head>
      <meta charset='UTF-8'>
      <meta http-equiv='X-UA-Compatible' content='IE=edge'>
      <meta name='viewport' content='width=device-width, initial-scale=1.0'>
      <link href='style.css' rel='stylesheet'>
      <title>Trebuchet</title>
    </head>
    <body>
      <div class='header'>
        <div class='grid'>
          <h1 class='site-heading'>Trebuchet</h1>
          <span class='g1'>
            <strong>Links:</strong><br>
            <strong>Headings:</strong><br><br><br>
            <strong>Bulleted List:</strong><br><br>
            <strong>Blockquote:</strong><br>
            <strong>Preformatted:</strong><br>
          </span>
          <span class='examples'>
            => gemini://example.com A cool gemlog<br>
            # Heading level 1<br>
            ## Heading level 2<br>
            ### Heading level 3 (that's it!)<br>
            * item 1<br>
            * item 2 etc...<br>
            > Shall I compare thee to a summer's day...<br>
            ```<br>
            fn example(arg):<br>
            &nbsp;&nbsp;print(arg)<br><br>
            ``` 
          </span>
        </div>
        <div id='flash'></div>
        <form id='post-content'>
          <textarea id='post-text' rows='2'></textarea>
        </form>
        <button form='post-content' type='submit' action='#'>Publish!</button>
      </div>
      <script src='./trebuchet.js'></script>
    </body>
    </html>")?;
    let mut css = fs::File::create("./web/style.css")?;
    css.write_all(b"body {
      background-color: #323232;
      color: #fff;
      font-family: 'Consolas', 'Courier New', Courier, monospace;
      font-size: 18px;
    }
    
    button {
      font-size: 16px;
      padding: 0.25em 0.5em;
      margin: 2em auto 2em 84%;
      background-color: #F49907;
      /* color: lightgreen; */
      border: 2px transparent solid;
    }
    
    @media screen and (min-width: 75em) {
      button {
        margin: 2em auto 2em calc(50% + 26em);
      }
    }
    
    #post-content {
      text-align: center; 
    }
    
    #post-text {
      width: 80%;
      max-width: 60em;
      padding: 0.5em;
      border: none;
      resize: none; 
      text-align: left; 
      font-size: 16px; 
      font-family: 'JetBrains Mono', 'Consolas', 'Courier New', Courier, monospace; 
      background-color: #464646;
      color: #F49907;
      outline: none;
    }
    
    #flash {
      text-align: center;
    }
    
    #flash div {
      margin: 1em auto;
      padding: 0.25em 0.5em;
      width: 58%;
      max-width: 52em;
      background-color: lightcoral;
    }
    
    .grid {
      display: grid;
      grid-template-columns: 10em auto;
      margin-bottom: 2em;
    }
    
    .g1 {
      grid-column: 1;
      text-align: right;
    }
    
    .examples {
      grid-column: 2;
      color: #F49907;
      padding-left: 1em;
    }
    
    .site-heading {
      grid-column: 2;
      color: #fff;
    }
    ")?;
    let mut js = fs::File::create("./web/trebuchet.js")?;
    js.write_all(b"function flash(msg) {
      let alert = document.querySelector('#flash').appendChild(document.createElement('div'))
      alert.textContent = msg
    }
    
    let form = document.querySelector('#post-content');
    let postText = document.querySelector('#post-text');
    
    form.addEventListener('submit', function( event) {
      let text = postText.value
      console.log(text)
      event.preventDefault()
    }, false)
    
    form.addEventListener('keyup', function(e) {
      // if key is Enter/Return key...
      if (e.keyCode == 13) {
    
        // remove any flash messages that are already visible
        let msgs = document.querySelector('#flash')
        if (msgs.hasChildNodes) {
          for (let n of msgs.childNodes) {
            msgs.removeChild(n)
          }
        }
    
        let gemPost = postText.value
        let arr = gemPost.split('\\n')
        postText.rows = arr.length // auto-resize to fit text
        var line = arr[arr.length - 2] // only check the last line we just created
    
        // check headings
        if (line.startsWith('#')) {
          if ( !(line.startsWith('# ') || line.startsWith('## ') || line.startsWith('### ')) ) {
            flash(`Is '${line}' at line ${arr.indexOf(line) +1} supposed to be a heading? Add a space after the '#' or make it level 1 to 3 only`)
          }
        }
    
        // check links
        if (line.startsWith('=>')) {
          let sections = line.split(' ')
          if ( !['http://', 'https://', 'gemini://', 'gopher://', 'mailto://', 'sftp://', 'ftp://', 'telnet://'].some(p => sections[1].startsWith(p))) {
            flash(`Your link '${line}' at line ${arr.indexOf(line) +1} looks a bit funny: did you put the URL and the text around the wrong way?`)
          }
        }
      }
    })")?;
    println!("✔   default files created");
    Ok(())
  }

  pub fn add_user(user: utils::User) -> Result<utils::User, sqlite::Error>{

    let connection = sqlite::Connection::open("trebuchet.db")?;
    // we need to borrow these values so we can return the user later
    let e = &user.email;
    let c = &user.capsule;
    // add the user to the db and return them
    let statement = connection.prepare("INSERT INTO users VALUES (:email, :capsule, '0')")?;
    let mut cursor = statement.into_cursor();
    cursor.bind_by_name(vec![
      (":email", sqlite::Value::String(e.to_string())), 
      (":capsule", sqlite::Value::String(c.to_string())),
      ])?;
      cursor.next()?;
    Ok(user)
  }

  pub fn delete_user(user: utils::User) -> Result<utils::User, sqlite::Error>{

    let connection = sqlite::Connection::open("trebuchet.db")?;
    // we need to borrow these values so we can return the user later
    let e = &user.email;
    let c = &user.capsule;
    // update the user confirmed value in the db and return them
    let statement = connection.prepare(
      "
      DELETE FROM users 
      WHERE email = :email AND home_directory = :capsule
      ")?;
    let mut cursor = statement.into_cursor();
    cursor.bind_by_name(vec![
      (":email", sqlite::Value::String(e.to_string())), 
      (":capsule", sqlite::Value::String(c.to_string())),
      ])?;
      cursor.next()?;
    Ok(user)
  }

  pub fn confirm_user(user: utils::User) -> Result<utils::User, sqlite::Error>{
    // TODO: check the TOKEN matches
    let connection = sqlite::Connection::open("trebuchet.db")?;
    // we need to borrow these values so we can return the user later
    let e = &user.email;
    let c = &user.capsule;
    // update the user confirmed value in the db and return them
    let statement = connection.prepare(
      "
      UPDATE users 
      SET confirmed = '1' 
      WHERE email = :email AND capsule = :capsule
      ")?;
    let mut cursor = statement.into_cursor();
    cursor.bind_by_name(vec![
      (":email", sqlite::Value::String(e.to_string())), 
      (":capsule", sqlite::Value::String(c.to_string())),
      ])?;
      cursor.next()?;
    Ok(user)
  }
}