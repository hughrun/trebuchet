use std::iter;
use rand::{Rng, distributions::Alphanumeric, thread_rng};
extern crate clap;
use clap::{Arg, App};

// ************************************************************
//  UTILITIES
// NOTE: this could probably go in a /src/lib.rs
// ************************************************************

struct User {
  email: String,
  directory: String,
  token: String
}

#[derive(Debug)]
enum EmailType {
  Confirm,
  LogIn
}

enum TrebuchetError {
  TokenError(String),
  EmailError(String)
}

impl User {
  fn send_email(self, email_type: EmailType) {
    // TESTING
    
    println!("emailing {} email with URL http://example.com/{:?}?token={}", self.email, email_type, self.token)

  }
  
  fn match_token(self) -> Result<Self,TrebuchetError>{
  
    // find token in DB
    // if no match look in expired_tokens table
    // if in expired_tokens check if used == true
    // if true, return TrebuchetError::TokenError("Token already used".to_string())
    // if false return TrebuchetError::TokenError("Token has expired".to_string())
    // if no match anywhere return TrebuchetError::TokenError("Token not recognised".to_string())
    // if both ok check email matches
    // if no match return TrebuchetError::EmailError("Email address not found")
    Ok(self)

  }

  fn initiate_login(self, etype: EmailType) {

    // set expiry date 
    // add token to DB - token, email, expiry

    // send email
    self.send_email(etype)
  }
}

fn build_user(email: &str, dir: &str) -> User {
  User {
    email: String::from(email),
    directory: String::from(dir),
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

// ************************************************************
//  DATABASE
// ************************************************************

// **********
//  INITIATE
// **********

// users - email, home_directory, confirmed (bool)
// tokens - token, email, expiry (datetime)
// expired_tokens - token, used (bool)
// files - owner, content, name, tags, type, is_draft (bool), published_date, updated_date, uses_footer (bool), uses_header (bool)

// **********
//  ADD USER
// **********

// **********
//  REMOVE USER
// **********

// **********
//  UPSERT FILE
// **********

// **********
//  DELETE FILE
// **********

// ************************************************************
//  WEBSITE
// ************************************************************

// NOTE: we can potentially gracefully save drafts locally (offline) first and then optionally to the server

// **********
//  INITIATE LOG IN
// **********


// **********
//  COMPLETE LOG IN
// **********

// get token and email address

// check match in DB and time has not expired

// MATCH => remove DB entry, set coookie and pass through to DASHBOARD
// NO MATCH OR EXPIRED => error page

// **********
//  COMPLETE EMAIL CONFIRMATION
// **********

// get token and email address

// check match in DB and time has not expired

// MATCH => remove DB entry, set CONFIRMED to TRUE, redirect to success page
// NO MATCH OR EXPIRED => error page

// ***********
//  EDIT POST
// ***********

// default view when logged in or error when not
// NEW button to create a new post - must provide post name
// EDIT button to update/edit an existing post - see VIEW POSTS)
// currently editing post is displayed in textarea
// metadata
//    - post or page (ie. does the date matter and is it in the feed?)
//    - name
//    - published (not editable?)
//    - tags (posts only)
//    - use header
//    - use footer

// ***********
//  DASHBOARD
// ***********

// display main options
//    - default header
//    - default footer
// retrieve posts from DB
// display name of each post in reverse-chronological order
// when click on name, display post text in EDIT POST view
// DELETE button - clicking requires confirmation

// **********
//  SAVE AS DRAFT
// **********

// get post text & metadata
// save to file data to LocalStorage and, if possible, to DB
// upsert tags with this post referenced
// save as published FALSE

// **********
//  PUBLISH
// **********

// get post text & metadata
// save to file data to DB
// upsert tags with this post referenced
// process site
//    - ignore content listed as published FALSE
//    - index (home) page: index list, latest post, neither?
//    - posts index page
//    - tag index page and index page for each tag
//    - header and footer (optionally)

// ************************************************************
//  CLI
// ************************************************************

// NOTE: all user subdirectories should automatically create a 'public' folder inside their directory which is where content is actually hosted. This allows us to delete the "default" user content without removing everything else.

// **********
//  INSTALL
// **********

// ask for screen inputs: 
//  - pages root directory (defaults to /srv/trebuchet/)
//  - initial user email
//  - initial user subdirectory (default none aka ./) <-- do no allow web users to alter this directly, too dangerous!
// initiate database
// create HTML files 
// create boilerplate test gemini page

// **********
//  CAPSULE
// **********

// arguments: 
//  - initial user email
//  - user subdirectory (default none) <-- do no allow web users to alter this directly, too dangerous!

fn add_user(mut args: clap::Values) {
  // we can use unwrap because clap will catch missing args
  let user = build_user(args.next().unwrap(), args.next().unwrap());
  // add user to database

  // send email to user
  user.initiate_login(EmailType::Confirm)

}

// display success/fail message

// send email to user with confirmation link

// diplay success/fail message

// **********
//  DELETE
// **********

// args: 
//  - initial user email
//  - user subdirectory 

// check email and subdirectory match

// screen input:
//  - ask for confirmation!
    // "You are about to delete user EMAIL and all gemini content in DIRECTORY"
    // yes =>
        // delete all entries in DB
        // delete all files and subdirectories (NEVER DELETE HOME DIRECTORY (i.e. 'none' directory should only delete 'public' folder))
        // display success/fail message
        // send email to user with message

// **********
//  USER
// **********

// arg - email_address OR directory
// print to screen
//    - email
//    - subdirectory
//    - number of files
//    - last published date

// **********
//  STATS
// **********

// print to screen
//    - total users
//    - total files
//    - last published date
//    - total storage?
//    - version of trebuchet?

fn main() {
  let matches = App::new("Trebuchet")
      .version("0.1.0")
      .author("Hugh Rundle <hugh@hughrundle.net>")
      .about("Publish and manage gemini sites from the web")
      .arg(Arg::with_name("install")
          .short("b")
          .long("build")
          .help("Set up a default user and web components")
          .takes_value(false)
          .conflicts_with_all(&["capsule", "delete", "user", "statistics"]))
      .arg(Arg::with_name("capsule")
          .short("c")
          .long("capsule")
          .help("Add a user with EMAIL address, whose gemini site will be saved to SUBDIRECTORY")
          .value_names(&["EMAIL", "SUBDIRECTORY"])
          .takes_value(true)
          .conflicts_with_all(&["install", "delete", "user", "statistics"]))
      .arg(Arg::with_name("delete")
          .short("d")
          .long("delete")
          .help("Remove a user and their files by providing their EMAIL address and the SUBDIRECTORY their files are saved to")
          .value_names(&["EMAIL", "SUBDIRECTORY"])
          .takes_value(true)
          .conflicts_with_all(&["install", "capsule", "user", "statistics"]))
      .arg(Arg::with_name("user")
          .short("u")
          .long("user")
          .help("Display details for a particular user with EMAIL address")
          .value_name("EMAIL")
          .takes_value(true)
          .conflicts_with_all(&["install", "delete", "capsule", "statistics"]))
          .arg(Arg::with_name("login")
          .short("l")
          .long("login")
          .help("Send login email")
          .takes_value(false)
          .requires("user")
          .conflicts_with_all(&["confirm", "install", "delete", "capsule", "statistics"]))
          .arg(Arg::with_name("confirm")
          .short("n")
          .long("confirm")
          .help("Send confirmation email")
          .takes_value(false)
          .requires("user")
          .conflicts_with_all(&["login", "install", "delete", "capsule", "statistics"]))
      .arg(Arg::with_name("statistics")
          .short("s")
          .long("statistics")
          .help("Display statistics about this trebuchet installation")
          .takes_value(false)
          .conflicts_with_all(&["install", "capsule", "user", "capsule"]))
      .get_matches();

  // it's ok to use unwrap here because clap ensures there will be values present
  if matches.is_present("install") {
    println!("I am installing!")
  }
  if matches.is_present("capsule") {
    println!("I am adding a user!");
    let args = matches.values_of("capsule").unwrap();
    // TESTING
    add_user(args)
  }
  if matches.is_present("delete") {
    println!("I am removing a user!")
  }
  if matches.is_present("user") {
    if matches.is_present("confirm") {
      println!("I am confirming user details!")
    } else if matches.is_present("login") {
      println!("I am sending a user login link!");
      build_user(matches.value_of("user").unwrap(), "").send_email(EmailType::LogIn)
    } else {
      println!("I am printing user details!")
    }
  }
  if matches.is_present("statistics") {
    println!("I am printing statistics!")
  }
}
