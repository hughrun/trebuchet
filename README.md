# Trebuchet: Web to Gemini publishing

Trebuchet is a web application for publishing [gemini](https://gemini.circumlunar.space/) capsules.

Trebuchet is neither server nor client software. It could be thought of as a very simple content management system. Trebuchet uses a sqlite database and writes gemfiles direct to the server file system, for hosting via Gemini server software.

## Rationale

Gemini has a lot of advantages, and has been carefully designed to be lightweight, simple, and essentially read-only. This is great for those of us who are comfortable with using a command line interface, SSH keys and all the rest, but it's a big barrier for people who either do not have access to these things (e.g. have only a locked-down company or school owned computer) or have neither the interest nor the time to learn how to use them. 

The [Gemini FAQ](https://gemini.circumlunar.space/docs/faq.gmi) states:

> Gemini already has a surprising number of client and server implementations in existence - which isn't to say more aren't welcome, but the real shortage right now is not of software but of content. **The more interesting and exciting stuff people find in Geminispace, the more likely they are to want to add content of their own.** So, the greatest contribution you can make to the project is to be a part of this process. 

I realise the irony of responding to this need with more software, however Trebuchet is an attempt to make it easer for more people to provide interesting Gemini content within a similar mental model as publishing to the web using a CMS like WordPress or a web app like Medium.

## Installing

### Build from source

```bash
git clone https://github.com/hughrun/trebuchet.git
cd trebuchet
cargo build --release
```

### Use the install script

```bash
curl -sSL https://raw.githubusercontent.com/hughrun/trebuchet/main/get-trebuchet.sh | bash
```

## Set up

Once you have installed Trebuchet, you will need to do some initial set up:

```bash
trebuchet build
```

This command will install, in the directory from where you ran the command:

1. a `web` directory containing the Trebuchet website files;
2. a `capsules` directory containing a default `index.gmi` file

## Commands

Trebuchet provides four commands for administrators.

### `capsule EMAIL NAME`

`trebuchet capsule` takes two arguments: `EMAIL`, and `NAME`, where the first is the user's email address, and the second is the name that will be used as the root directory for this capsule. `NAME` must be ASCII alphanumeric.

A new capsule directory will be created, and the user will be sent an email with a confirmation link which must be used before they can publish.

### `delete EMAIL NAME`

`trebuchet delete` takes two arguments: `EMAIL`, and `NAME`, where the first is the user's email address, and the second is the name of the root directory for the capsule.

If the email address and name match, an interactive prompt will ask for confirmation. On confirmation, the capsule files and related database entries will be deleted. This is a non-recoverable action!

### `user EMAIL [--confirm | --login]`

`trebuchet user` takes one argument: `EMAIL`, which is the email address of the user.

Information about the user will be printed to the console.

Two optional flags are available with the `user` command:

* the `--confirm` flag will trigger a confirmation email. This will overwrite any outstanding confirmation token triggered by `capsule`. You may need to use this to trigger a new email if the old token expired or the email was not delivered for some reason.
* the `--login` flag will trigger a login email, the same as would normally be triggered by a login attempt using the web interface.

### `statistics`

`trebuchet statistics` takes no arguments. It prints general information about the capsules in this installation, to the console.

## Administering Trebuchet

Trebuchet is designed to enable authors to publish content without direct access to a server. It tries not to have many opinions about how the server is administered, other than expecting a Unix-like file system and POSIX shell. Since it enables interaction between the two protocols, Trebuchet requires the host server to have both web server software (e.g. Apache, nginx, or Caddy) _and_ [Gemini server software](https://gemini.circumlunar.space/software) (e.g. Agate or Jetforce)&mdash;but it doesn't care which server software you use in either case.

Your web server should serve files from the `web` directory. 

For the Gemini server, you have more options. Trebuchet will build capsules using the following file structure:
```
root_directory
  - username
    - index.gmi
    - page-name
    - index.gmi
    - YYYY-MM-DD
      - name-of-post
        - index.gmi
      - name-of-second-post-same-day
        - index.gmi
    - YYYY-MM-DD
      - name-of-another-post-different-day
        - index.gmi
```
How these capsules are actually served is up to the server administrator. The two most obvious options are to either

1. serve each capsule independently at a separate domain with the source directory being `capsules/username` and the url as `gemini://example.com`;
2. serve the entire collection from the same domain using the source directory `capsules`, with each capsule served from `gemini://example.com/username`.

How you set up your server will depend on why you are using Trebuchet and what you hope to achieve.

## Publishing with Trebuchet

Unlike server administration, but like the Gemini protocol itself, Trebuchet is highly opinionated about how authors can publish capsules. It provides some flexibility where useful, but does not provide users with a large and confusing range of configuration options. There are, after all, only so many ways one can publish a Gemini capsule.

Authors log in at `https://example.com/` by entering their email address, and then clicking on the one-time link that will be sent to them at that address. Trebuchet uses single-use tokens rather than storing passwords. Cookies to enable authenticated sessions are relatively short-lived, though drafts are saved locally to avoid data loss.

Once logged in, authors can either draft a new document (post/page), or edit an existing one, within the web interface. It is not possible to create new users or capsules from the web interface, nor to delete them from the web interface&mdash;this is deliberate to reduce both the possible attack surface and the complexity required by the application.

### Posts and pages

All files are either a _post_ (default) or _page_. Posts included in Gemini subscription lists (see [tags and content lists](#tags-and-content-lists), and [includes](#includes) below), whereas pages are not. 

Posts will have a URL of `/{YYYY-MM-DD}/{hyphenated-title}` whereas pages are found at `/{hyphenated-title}`

### Tags and content lists

Both _posts_ and _pages_ can be assigned _tags_, which perform the same function as in typical blogging or CMS software&mdash;small pieces of metadata to describe the topic or purpose of the document. Trebuchet automatically builds index pages for all _posts_ and all _tags_, using the convention described in [Subscribing to Gemini pages](https://gemini.circumlunar.space/docs/companion/subscription.gmi). This allows Geminauts to subscribe to a whole capsule and/or certain tags.

An index page listing all posts is saved as `archive.gmi`. Index pages for each tag are saved as `tagged-tagname.gmi`, and list every post that uses that tag. Since posts can have multiple tags and every post appears at `archive.gmi`, a single post will appear on multiple index pages.

### Includes

If you have used a static capsule generator or templating system before you will be familiar with the concept of "includes". These are basically files that are added to a page prior to rendering, generally indicated by using a "shortcode" or metadata tag in the page template. Trebuchet currently has only three _include codes_, using the "handlebars" style, as well as an automatically included _header_ and _footer_.

1. Headers and footers

Trebuchet includes a header and a footer on every page and post by default. These are saved for each user in the database (but not as separate files) as `includes.footer` and `includes.header`, and may contain any valid gemtext or include codes. Headers and footers can be excluded for individual posts or pages. To exlude the header and/or footer for all posts and pages, simply leave `includes.header` and/or `includes.footer` blank.

2. {{ latest }}

The `{{ latest }}` code creates a link list of the ten most recently published _posts_, using the convention described in [Subscribing to Gemini pages](https://gemini.circumlunar.space/docs/companion/subscription.gmi). This can be used on a homepage to provide a rolling lighweight Gemini feed listing.

3. {{ tag._tagname_ }}

The `{{ tag.tagname }}` code creates a link list of all pages or posts that are tagged with `tagname`. This can be used, for example, to create a standard footer with links to all the _pages_ within a gemini capsule, that will automatically update to include any new pages that are added, if pages are all tagged with something like "`pages`", or to include a list of "sticky" or "highlighted" posts on a homepage. Unlike `{{ latest }}` and tag index pages, tag lists created this way do not include a date and are not designed to be used for Gemini subscription pages.

4. {{ tagsList }}

The default footer includes the `{{ tagsList }}` include code. This will create a linked list of all tags used on the post or page (with each link pointing to the index page). This code can also be used anywhere within the text of a page or post. If you want to display tags lists in the footer for posts but not pages, you need to deselect _show footer_ in each _page_ as there is no logic available for include codes.

### Drafts, publishing, and updating

Content can be saved as a _draft_, or published as a _post_ or _page_.

When in editing mode, content is auto-saved as a draft every 5 minutes. If you are offline, the draft will be saved to [LocalStorage](https://developer.mozilla.org/en-US/docs/Web/API/Web_Storage_API) so that you don't lose your work, and saved to the server the next time it is available.

When a file is _published_ as a _post_ or a _page_, the content and metadata is read from the database and:

1. the content is written to a file;
2. any index pages for tags or the post archive are written to file

Although it does not serve content dynamically, Trebuchet considers the database to be the "single source of truth". Any files manually edited or added to a capsule directory are therefore liable to be overwritten. Updates should therefore always be made via the web interface.

Posts and pages can be edited/updated after publication. In this case the published date will not change, though a "last edited" date is recorded in the database.

### Limitations

Currently each email address can only have one capsule associated with it. This may change in future.

Trebuchet is a text-only publishing platform and currently there are no plans to enable file uploads (e.g. for storing image files that can be linked to in Gemini pages).