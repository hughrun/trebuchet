#![allow(dead_code)]

pub mod error {
  use std::{fmt, io};

  #[derive(Debug)]
  pub enum TrebuchetErrorType {
    EmailError,
    IoError,
    NotFound,
    SqliteError,
    TooManyMatches,
    TokenError
  }

  #[derive(Debug)]
  pub struct TrebuchetError {
    pub kind: TrebuchetErrorType,
    pub message: String,
  }

  impl fmt::Display for TrebuchetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match &self.kind {
          TrebuchetErrorType::EmailError => "Error sending email",
          TrebuchetErrorType::IoError => "Error from IO process",
          TrebuchetErrorType::NotFound => "No rows match in database",
          TrebuchetErrorType::SqliteError => "sqlite returned an error",
          TrebuchetErrorType::TooManyMatches => "Too many matches in database",
          TrebuchetErrorType::TokenError => "Error checking token"
        };

        write!(f, "{}", err_msg)
    }
}

  // Implement std::convert::From for TrebuchetError; from io::Error
  impl From<io::Error> for TrebuchetError {
    fn from(error: io::Error) -> Self {
      TrebuchetError {
        kind: TrebuchetErrorType::IoError,
        message: error.to_string(),
      }
    }
  }

  // Implement std::convert::From for TrebuchetError; from sqlite::Error
  impl From<sqlite::Error> for TrebuchetError {
    fn from(error: sqlite::Error) -> Self {
    TrebuchetError {
        kind: TrebuchetErrorType::SqliteError,
        message: error.to_string(),
      }
    }
  }

  pub fn build_token_error(msg: String) -> TrebuchetError {
    TrebuchetError {
      kind: TrebuchetErrorType::TokenError,
      message: msg
    }
  }

}

pub mod utils {

  use std::{fs::File, io, iter};
  use rand::{Rng, distributions::Alphanumeric, thread_rng};
  use crate::database;
  use crate::error::{TrebuchetError, build_token_error};

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

// Orphaned Functions
// =================

  pub fn file_exists(path: &str) -> std::io::Result<()> {
    let _f = File::open(path)?;
    Ok(())
  }

  pub fn hyphenate( tag: String ) -> String {
    let mut alphanum = tag;
    alphanum.retain(|ch: char| ch.is_ascii_alphanumeric() || ch == ' ');
    let downcased = alphanum.to_lowercase();
    downcased.replace(|ch: char| ch == ' ', "-")
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

    pub fn new(email: String, capsule: String) -> User {
      User { email, capsule, token: create_otp() }
    }

    // TODO: should this be Box or TrebuchetError?
    pub fn add(self) -> Result<(), Box<(dyn std::error::Error)>>{

      // add user to database and send email
      database::add_user(self)?.initiate_login(EmailType::Confirm)?;
      Ok(())
    }

    pub fn confirm(self) -> Result<(), TrebuchetError>{

      // TODO: add user files

      // update database and send email
      database::confirm_user(self)?.initiate_capsule()?.build_email(EmailType::Confirm)?;
      Ok(())
    }

    pub fn delete(self) -> Result<(), Box<(dyn std::error::Error)>> {
      
      // TODO: delete user files

      // remove user from database and send email
      database::delete_user(self)?.build_email(EmailType::Delete)?;
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
    // FIXME: should not be public - only for testing
    pub fn initiate_capsule(self) -> Result<User, TrebuchetError> {

      // initiate default values in DB
      database::initiate_capsule(self)
    }

    // PRIVATE FUNCTIONS
    // ----------------

    fn build_email(self, email_type: EmailType) -> Result<(), io::Error> {
      // create URL
      let root_domain = "https://example.com"; // TODO: this needs to be an ENV value
      let link = format!("{}/{:?}?token={}", root_domain, email_type, self.token);

      // email text templates
      let confirmation_email_text = format!("Hello!\n\nYou are being invited to publish a Gemini capsule named {}, via {}.\n\nOpen the link below to confirm.\n\n<a href=\"{}\">{}</a>", self.capsule, root_domain, link, link);

      let deletion_email_text = format!("Hello\n\nYour Gemini capsule named {}, served from {}, has been deleted.", self.capsule, root_domain);

      let login_email_text = format!("Hello!\n\nYou or someone else initiated a login at {}.\n\nOpen the link below to complete your login.\n\n<a href=\"{}\">{}</a>\n\nIf this was not you, ignore this email or advise your server administrator.", root_domain, link, link);

      // send email according to email_type
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
      let row = User {email: "me@me.com".to_string(), capsule: "cap".to_string(), token: "123abc".to_string()};
      let token_matches = self.token == row.token;
      match token_matches {
        true => Ok(self),
        false => {
          // TODO: replace this with a check of the DB that returns a Token if something is found and nothing if not
          // TESTING:
          let used = true; // we get this from the DB: was the token used or not?
          let unmatched_token = Some(used); // this is what we actaully return from the DB checking function we need to create. It will either return Some(token.used) or None

          // check whether anything was returned
          // if None, there was no matching token at all (this should never happen)
          // otherwise the token either was been used already or has expired
          match unmatched_token {
            Some(t) => {
              match t {
                true => Err(build_token_error("Token already used".to_string())),
                false => Err(build_token_error("Token has expired".to_string()))
              }
            },
            None => Err(build_token_error("Token not recognised".to_string()))
          }
        }
      }
    }
  }
}

pub mod database {
  use crate::utils;
  use crate::error;
  use chrono::{Local, Utc};
  use std::{collections::HashMap, fmt, fs, io::prelude::*};
  use sqlite;
  
  pub enum ContentType {
    Draft,
    Include,
    Page,
    Post
  }

  impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c_type = match &self {
          ContentType::Draft => "draft",
          ContentType::Include => "include",
          ContentType::Page => "page",
          ContentType::Post => "post"
        };

        write!(f, "{}", c_type)
    }
}

  pub struct Document {
    owner: String,
    title: String,
    tags: Vec<String>,
    published: String,
    updated: String,
    content: String,
    header: bool,
    footer: bool,
    content_type: ContentType
  }

  pub fn build_tables() -> Result<(), sqlite::Error>{

    let connection = sqlite::Connection::open("trebuchet.db")?;

    connection.execute(
        "
        CREATE TABLE users (email TEXT UNIQUE, home_directory TEXT UNIQUE, confirmed INTEGER);
        CREATE TABLE tokens (token TEXT PRIMARY KEY, email TEXT, expiry TEXT);
        CREATE TABLE expired_tokens (token TEXT PRIMARY KEY, email TEXT, used INTEGER);
        CREATE TABLE cookies (user INTEGER, expiry TEXT);
        CREATE TABLE documents (owner TEXT, content TEXT, title TEXT, tags TEXT, type TEXT, published_date TEXT, last_updated TEXT, uses_footer INTEGER, uses_header INTEGER, UNIQUE(owner, title));
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
    // NOTE: don't create anything in the default directory because it might be overwritten by a single default user on creation!
    // let mut gem = fs::File::create("./capsules/index.gmi")?;
    // gem.write_all(b"# Home\n\nWelcome to my site built with Trebuchet!\n\n=> gemini://trebuchet.hugh.run Find out more about Trebuchet")?;

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

      // TODO: add token to tokens collection

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

  pub fn confirm_user(user: utils::User) -> Result<utils::User, error::TrebuchetError> {
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
      WHERE email = :email AND home_directory = :capsule
      ")?;
    let mut cursor = statement.into_cursor();
    cursor.bind_by_name(vec![
      (":email", sqlite::Value::String(e.to_string())), 
      (":capsule", sqlite::Value::String(c.to_string())),
      ])?;
    cursor.next()?;

    // sqlite returns a success code of 0 if the operation could have done something, even if it doesn't i.e. if nothing matches.
    // we therefore need to check how many rows changed and return a custom error
    let rows_affected = connection.change_count();
    match rows_affected {
      1 => Ok(user),
      0 => Err(error::TrebuchetError {
        kind : error::TrebuchetErrorType::NotFound, 
        message : String::from("No matching rows found")
      }),
      _ => Err(error::TrebuchetError {
        kind : error::TrebuchetErrorType::TooManyMatches, 
        message : String::from("More than one row matches but only one row should!")
      })
    }
  }

  fn create_document(email: &String, title: String, tags: Vec<String>, content: String, content_type: ContentType) -> Document {
    let doc = Document {
      owner: email.to_string(),
      title,
      tags,
      published: Local::now().format("%Y-%m-%d").to_string(),
      updated: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
      content,
      header: true,
      footer: true,
      content_type
    };
    doc
  }

  pub fn initiate_capsule(user: utils::User) -> Result<utils::User, error::TrebuchetError> {
    // TODO:

    // in all cases error if already exists

    // initiate includes.footer content
    // default footer to include links to home, archive, orbit, and Trebuchet itself
    let footer = String::from("\n-------\n{{ tags-list }}\n=> /index.gmi Home\n=> /archive Archive\n=> /orbit Other capsules in my orbit\n=> gemini://trebuchet.hugh.run Made with Trebuchet\n");
      let footer_doc = create_document(&user.email, "includes.footer".to_string(), Vec::new(), footer, ContentType::Include);
      save_content(footer_doc)?;
    // initiate index.gmi with default content (including shortcode)
    let index = String::from("# My Gemini Capsule\n\nWelcome to my Gemini capsule, published with Trebuchet.\n\n{{ latest }}\n");
    let index_doc = create_document(&user.email, "index.gmi".to_string(), Vec::new(), index, ContentType::Include);
    save_content(index_doc)?;
    // initiate orbit.gmi for gemini capsules in my orbit (i.e. equivalent to a blogroll)
    let orbit = String::from("# Other Gemini capsules in my orbit\n\n=> gemini://gemini.circumlunar.space/capcom CAPCOM: an aggregator for Atom feeds of Gemini content\n=> gemini://trebuchet.hugh.run Trebuchet: a web application for publishing Gemini capsules\n");
    let orbit_doc = create_document(&user.email, "Orbit".to_string(), Vec::new(), orbit, ContentType::Page);
    save_content(orbit_doc)?;

    publish_capsule(user)
  }

  fn save_content(doc: Document) -> Result<(), error::TrebuchetError> {

    let connection = sqlite::Connection::open("trebuchet.db")?;

    let mut uses_footer = 0;
    if let false = doc.footer { uses_footer = 1 };

    let mut uses_header = 0;
    if let false = doc.header { uses_header = 1 };

    let statement = connection.prepare(
      "
      INSERT INTO documents 
      VALUES (:owner, :content, :title, :tags, :type, :published_date, :last_update, :uses_footer, :uses_header)
      ")?;
    let mut cursor = statement.into_cursor();
    cursor.bind_by_name(vec![
      (":owner", sqlite::Value::String(doc.owner)),
      (":content", sqlite::Value::String(doc.content)),
      (":title", sqlite::Value::String(doc.title)), 
      (":tags", sqlite::Value::String(doc.tags.join(":::"))), // CHECK:
      (":type", sqlite::Value::String(doc.content_type.to_string())), 
      (":published_date", sqlite::Value::String(doc.published)), 
      (":last_update", sqlite::Value::String(doc.updated)), 
      (":uses_footer", sqlite::Value::Integer(uses_footer)), 
      (":uses_header", sqlite::Value::Integer(uses_header))
      ])?;
      cursor.next()?;

    Ok(())
  }

  fn update_document() {
    // TODO:
  }

  // TODO:
  fn publish_capsule(user: utils::User) -> Result<utils::User, error::TrebuchetError> {

    let connection = sqlite::Connection::open("trebuchet.db")?;
    // get all documents belonging to this user
    let mut cursor = connection.prepare(
      "SELECT * FROM documents 
       WHERE owner = :user
       ORDER BY type ASC, published_date DESC;
      ")?;
    cursor.bind_by_name(":user", user.email.as_str())?;

    // TODO: now we process them all

    let mut footer = String::new();
    let mut header = String::new();
    let mut index = String::new();
    let mut pages: HashMap<String, String> = HashMap::new();
    let mut posts: HashMap<String, String> = HashMap::new();
    let mut latest = String::new();
    let mut tags: HashMap<String, Vec<String>> = HashMap::new();

    while let sqlite::State::Row = cursor.next()? {
      // get includes
      let content = format!("{}", cursor.read::<String>(1)?); // content
      let title = format!("{}", cursor.read::<String>(2)?); // title is always the same for includes files
      let tags_string = format!("{}", cursor.read::<String>(3)?);
      let c_type = format!("{}", cursor.read::<String>(4)?); // content type
      let published = format!("{}", cursor.read::<String>(5)?); // published date

      if title == String::from("includes.footer") {
        footer.push_str(content.as_str());
      }
      if title == String::from("includes.header") {
        header.push_str(content.as_str());
      }
      if title == String::from("index.gmi") {
        index.push_str(content.as_str());
      }
      // for each page,
      if c_type == String::from("page") {
        // add header and footer
        let page = format!("{}\n{}\n{}", &header, content, &footer);
        // push to vec
        pages.insert(utils::hyphenate(title.to_owned()), page);
        // pages.insert(title.to_owned(), page);
      }
      // for each post ordered by publication date
      if c_type == String::from("post") {
        // add header and footer
        let post = format!("{}\n{}\n{}", &header, content, &footer);
        // insert to hashmap
        // key is YYYY-MM-DD-hyphenated-title
        let k = format!("{}-{}", published, utils::hyphenate(title.to_owned()));
        posts.insert(k, post);
        // if in first 10 posts ordered by date descending
        if cursor.column_count() < 10 {
          // append to a String to insert into {{ latest }}
            let listing = format!("=> /{}-{} {} - {}\n", published, title, published, title);
          latest.push_str(listing.as_str());
        }
      }

      // get all tags and build tags_list for tag-tagname archive pages
      let doc_tags: Vec<&str> = tags_string.split(":::").collect();
    // NOTE 1: this seems inefficient with clones and to_owned but not sure how else to do it
    for tagname in doc_tags {
        if tags_string.len() > 0 {
          // NOTE 2: this will probably be a useful pattern for {{ latest }} and {{ tag.tagname }}
          // can it be a full util function or a closure?
          let ot = tagname.to_owned();
          let t = utils::hyphenate(ot); // hyphenate and ascii downcase
          let listing;
          if c_type == "post" {
            listing = format!("=> /{}-{} {} - {}\n", published, utils::hyphenate(title.clone()), published, title);
          } else {
            listing = format!("=> /{} {}\n", utils::hyphenate(title.clone()), title);
          }
          match tags.get(&t) {
            Some(entries) => {
              let mut en = entries.clone();
              en.push(listing.clone());
              tags.insert(t, en)
            }
            None => tags.insert(t.to_owned(), vec![listing])
          };
        };
      };
    }

    // NOW WRITE OUT FILES
    // fs::create_dir_all creates home directory at ./capsules/content/{local.capsule}
    // this allows us to do things like if local.capsule is a domain (www.example.com), agate (or whatever) will serve from that domain
    // or if it's just a username or something (~hugh-is-on-gemini), that's fine too and it becomes a path within the base domain
    // TAGS
    // for each tag...
    for (t, v) in tags {
      let mut tagpage = String::new();
      // for post in the vec of posts...
      for p in v {
        tagpage.push_str(&p)
      }
      // write out file at {tag-tagname}/index.gmi
      let tag_path = format!("./capsules/content/{}/tag-{}/index.gmi", user.capsule, t);
      // this will error if already exists, which is what we want
      fs::create_dir_all(format!("./capsules/content/{}/tag-{}", user.capsule, t))?;
      fs::File::create(&tag_path)?;
      fs::write(tag_path, tagpage)?;
    }

    // TODO: {{ latest }}
    // TODO: {{ tags-list }}
    // TODO: add posts to archive page

    // PAGES
    // TODO: make this DRY
    // create a file at {hyphenated-title}/index.gmi
    for (t, c) in pages {
      fs::create_dir_all(format!("./capsules/content/{}/{}", user.capsule, t))?;
      let page_path = format!("./capsules/content/{}/{}/index.gmi", user.capsule, t);
      fs::File::create(&page_path)?;
      fs::write(page_path, c)?;
    }

    // POSTS
    // create a file at {DATE}-hyphenated-title/index.gmi
    for (k, content) in posts {
      fs::create_dir_all(format!("./capsules/content/{}/{}", user.capsule, k))?;
      let post_path = format!("./capsules/content/{}/{}/index.gmi", user.capsule, k);
      fs::File::create(&post_path)?;
      fs::write(post_path, content)?;
    }
    // INDEX
    // create a file at index.gmi
    // directory already exists
    let index_path = format!("./capsules/content/{}/index.gmi", user.capsule);
    fs::File::create(&index_path)?;
    fs::write(index_path, index)?;
    Ok(user)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  // ERROR MODULE
  // ============
  // TODO: how do we test that TrebuchetError successfully implements sqlite::Error and io::Error?

  // UTILS MODULE
  // ============
  // TODO: database functions from utils - how do we mock them?

  #[test]
  fn utils_build_user_returns_user() {
    let built_user = utils::User::new("hello@email.com".to_string(), "capsule_name".to_string());
    let user_two = utils::User {
      email: "hello@email.com".to_string(),
      capsule: "capsule_name".to_string(),
      token: "randomstring".to_string()
    };

    assert_eq!(built_user.email, user_two.email);
    assert_eq!(built_user.capsule, user_two.capsule);
  }

  #[test]
  #[ignore]
  fn utils_builds_confirmation_email() {
    assert!(false)
  }

  #[test]
  #[ignore]
  fn utils_builds_login_email() {
    assert!(false)
  }

  #[test]
  #[ignore]
  fn utils_builds_deletion_email() {
    assert!(false)
  }

  #[test]
  #[ignore]
  fn utils_send_email_sends_email() {
    assert!(false)
  }

  #[test]
  #[ignore]
  fn utils_match_token_matches_token() {
    assert!(false)
  }

  // DATABASE MODULE
  // ===============
  // TODO: database module tests

  // DANGER: this does live changes to the DB
  #[test]
  #[ignore]
  fn confirmation_test() {
    utils::User::new("molly@dog.dog".to_string(),"dogger".to_string()).add().unwrap();
    match utils::User::new("molly@dog.dog".to_string(),"dogger".to_string()).initiate_capsule() {
      Ok(_user) => assert!(true),
      Err(e) => {
        eprintln!("{}", e.message);
        assert!(false)
      }
    }
  }
}