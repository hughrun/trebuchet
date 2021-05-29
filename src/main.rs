use std::process::Command;
use trebuchet::utils::{build_user, EmailType, file_exists};
use trebuchet::database;
use clap::{Arg, App};

// ************************************************************
//  DATABASE
// ************************************************************

// **********
//  BUILD
// **********

fn build() {
  // check that sqlite is installed
  println!("Checking for sqlite...");
  let sqlite_check = Command::new("which")
  .arg("sqlite3")
  .status()
  .expect("'which' command failed when checking for sqlite3");

  if sqlite_check.success() {
    println!("✔   sqlite installed");
    // check whether there is already a database
    let installed = file_exists("./trebuchet.db");
    // if there is, throw error
    match installed {
      Ok(()) => eprintln!("⚠️  database already exists"),
      Err(err) => {
        if err.kind() == std::io::ErrorKind::NotFound {
            // otherwise create new DB file
            match database::build_tables() {
              Ok(()) => {
                println!("✔   database created");
                // check for web and capsule directories
                match database::create_default_files() {
                  Ok(()) => { 
                    println!("😎  You are ready to use Trebuchet")
                },
                  Err(err) => eprintln!("Error creating default files: {}", err)
                }
                // if neither exists, create them and write out files
              },
              Err(err) => eprintln!("Error creating database: {}", err)
            }
        } else {
          eprintln!("⚠️  Cannot read DB: {}", err)
        }
      }
    }
  } else {
    println!("⚠️  sqlite must be installed to run Trebuchet. See https://www.sqlite.org for options")
  }
}


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
      .arg(Arg::with_name("build")
          .short("b")
          .long("build")
          .help("Set up a default user and web components")
          .takes_value(false)
          .conflicts_with_all(&["capsule", "delete", "listen", "user", "statistics"]))
      .arg(Arg::with_name("capsule")
          .short("c")
          .long("capsule")
          .help("Add a user with EMAIL address, whose gemini site will be saved to SUBDIRECTORY")
          .value_names(&["EMAIL", "SUBDIRECTORY"])
          .takes_value(true)
          .conflicts_with_all(&["build", "delete", "listen", "user", "statistics"]))
      .arg(Arg::with_name("delete")
          .short("d")
          .long("delete")
          .help("Remove a user and their files by providing their EMAIL address and the SUBDIRECTORY their files are saved to")
          .value_names(&["EMAIL", "SUBDIRECTORY"])
          .takes_value(true)
          .conflicts_with_all(&["build", "capsule", "user", "statistics"]))
      .arg(Arg::with_name("listen")
          .short("L")
          .long("listen")
          .help("Listen for web traffic")
          .takes_value(false)
          .conflicts_with_all(&["build", "capsule", "delete", "listen", "user", "statistics"]))
      .arg(Arg::with_name("user")
          .short("u")
          .long("user")
          .help("Display details for a particular user with EMAIL address")
          .value_name("EMAIL")
          .takes_value(true)
          .conflicts_with_all(&["build", "delete", "listen", "capsule", "statistics"]))
          .arg(Arg::with_name("login")
          .short("l")
          .long("login")
          .help("Send login email")
          .takes_value(false)
          .requires("user")
          .conflicts_with_all(&["confirm", "build", "delete", "listen", "capsule", "statistics"]))
          .arg(Arg::with_name("confirm")
          .short("n")
          .long("confirm")
          .help("Send confirmation email")
          .takes_value(false)
          .requires("user")
          .conflicts_with_all(&["login", "build", "delete", "listen", "capsule", "statistics"]))
      .arg(Arg::with_name("statistics")
          .short("s")
          .long("statistics")
          .help("Display statistics about this trebuchet installation")
          .takes_value(false)
          .conflicts_with_all(&["build", "capsule", "user", "capsule"]))
      .get_matches();

  // it's ok to use unwrap here because clap ensures there will be the required a present
  if matches.is_present("build") {
    build()
  }
  if matches.is_present("listen") {
    // TODO: ideally this runs in the background automatically thought that could perhaps better be a systemd service
    println!("I am listening for web traffic...")
  }
  if matches.is_present("capsule") {
    let args: Vec<&str> = matches.values_of("capsule").unwrap().collect();
    build_user(args[0], args[1]).add_user()
  }
  if matches.is_present("delete") {
    // TODO: remove underscore and uncomment once delete_user() is ready
    let _args: Vec<&str> = matches.values_of("delete").unwrap().collect();
    // build_user(args[0], args[1]).delete_user()
  }
  if matches.is_present("user") {
    if matches.is_present("confirm") {
      build_user(matches.value_of("user").unwrap(), "").initiate_login(EmailType::Confirm)
    } else if matches.is_present("login") {
      build_user(matches.value_of("user").unwrap(), "").initiate_login(EmailType::LogIn)
    } else {
        println!("I am printing user details!")
    }
  }
  if matches.is_present("statistics") {
      println!("I am printing statistics!")
  }
}
