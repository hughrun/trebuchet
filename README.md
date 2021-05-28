# Trebuchet: web to gemini publishing

Trebuchet is a web application for publishing [gemini](https://gemini.circumlunar.space/) capsules.

Trebuchet is neither server nor client software. It could be thought of as a very simple content management system. Trebuchet uses a sqlite database and writes gemfiles direct to the server file system, for hosting via Gemini server software.

## Rationale

Gemini has a lot of advantages, and has been carefully designed to be lightweight, simple, and essentially read-only. This is great for those of us who are comfortable with using a command line interface, SSH keys and all the rest, but it's a big barrier for people who either do not have access to these things (e.g. have only a locked-down company or school owned computer) or have neither the interest nor the time to learn how to use them. 

The [Gemini FAQ](https://gemini.circumlunar.space/docs/faq.gmi) states:

> Gemini already has a surprising number of client and server implementations in existence - which isn't to say more aren't welcome, but the real shortage right now is not of software but of content. **The more interesting and exciting stuff people find in Geminispace, the more likely they are to want to add content of their own.** So, the greatest contribution you can make to the project is to be a part of this process. 

I realise the irony of responding to this need with more software, however Trebuchet is an attempt to make it easer for more people to provide interesting Gemini content within a similar mental model as publishing to the web using a CMS like WordPress or a web app like Medium.
