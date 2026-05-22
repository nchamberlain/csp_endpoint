# A Simple Content Security Policy Endpoint
## Intro
The modern internet has been afflicted by a number of behind-the-scenes security threats, ranging from minor nuisances to major misfortunes. To combat these threats, a variety of security directives have been implemented that modern browsers are supposed to follow.

## Background
The http protocol is the backbone of the internet. It defines the request and response mechanism that allows browsers to interact with websites. At its most basic level, a user (browser) requests an operation to be performed at a URL (web address) and the URL responds with the appropriate status code and any other matching content.

To meet the ever-growing demand for user-friendliness, a variety of tools were added to the website designer’s toolbox, such as scripting languages and other dynamic enhancements, such as css. As the power of the designer’s toolbox grew, so did the capabilities of non-benevolent actors who were more interested in achieving their own goals than what was in the best interests of the user. Think clickjacking and cross-site scripting that could lead to identity and financial theft.

To deal with these threats, a number of security directives have been developed, such as HSTS (HTTP Strict Transport Security) and CSP (Content Security Policy) settings. These directives are not intended to be the first line of defense but are intended to be part of a defense-in-depth, multi-layered approach to security. 

A detailed description of these security threats and their mitigation are beyond the scope of this document. For more information, the documentation from the W3C and OWASP (Open Worldwide Application Security Project) are recommended. The [W3C Working Draft of the Content Security Policy Level 3](https://www.w3.org/TR/CSP3/#cspro-header) defines “a mechanism by which web developers can control the resources which a particular page can fetch or execute, as well as a number of security-relevant policy decisions” explains these threats and ways to mitigate. The [OWASP Cheat Sheet Series Project](https://cheatsheetseries.owasp.org/index.html) “was created to provide a concise collection of high value information on specific application security topics” and offers more than 100 cheat sheets. 

The [OWASP Top Ten](https://cheatsheetseries.owasp.org/IndexTopTen.html) is highly recommended for reading because it “represents a broad consensus about the **most critical security risks** to web applications.” (emphasis added by this document’s author). The Top Ten actually covers the top 10 categories of critical security risks, such as Broken Access Control (4 cheat sheets), Cryptographic Failures (6 cheat sheets), Injection (10 cheat sheets) and 7 more. Still big, but at least it limits the scope somewhat and allows us to focus on the most important issues.

## Running the Code Locally
Install Rust per the commonly available instructions. Clone this project and switch to that directory. The code is designed to run localhost:8088/fnirc7 - edit the code in src/main.rs to point to a preferred port and path if desired. From the terminal/command line, enter:

   cargo run 
 
and it will build the project and then start the csp endpoint (Actix-web server). As described under Project Design Considerations, the endpoint can only receive posts. As discussed in that section, for security reasons, there are no methods of retrieving the posted data outside of the server. 

If you feel comfortable retrieving it from your browser, you can uncomment the get function in main.rs and configure it as required.
This project also contains code for exercising the post capabilities. In src/bin is file reqs.rs that builds various posts that test various aspects of the server. From a different command line (terminal window), enter:

   cargo run --bin reqs

The reqs.rs file does the following tests:
* the first post attempts to send a message that is more than 10,000 characters long. That post also contains some special characters, such as an accented e and some Japanese words. The endpoint makes the assumption that messages greater than 2,000 characters is probably some hacking attempt at a buffer overflow so the server truncates the message after 1,000 characters (not bytes). Using chars and not bytes helps to prevent panics caused by breaking in the middle of a multi-byte character from non-Latin sources (emojis, foreign languages, etc).
* the next post is a more typical (and shorter) message
* the next message sends formatting/control characters commonly in Rust strings, such as tabs or newlines in the received message.
* the final message sends all of the unicode control characters are specifically tested, including 0, 127 and all of the characters from 128 to 159 (x80 - x9F)

Both the csp endpoint and reqs log results. The logging messages are displayed as plain text in the terminal/command window. If desired, set the default logging level to debug to display some more information.

## Project Design Considerations
CSP documentation frequently mentions CSP Reporting Endpoints that are to receive information about security and CSP directives violations but they rarely mention what these endpoints are and how they function. There’s just vague statements that assume that they will be there to receive the violation messages.

After some research, it was determined that these endpoints are “out-of-band” (conceptually independent channels separate from the website) receivers of the violation reports. Additional research yielded the detail that the reports are expected to be in JSON format but a standard JSON format for the reports was not found. There may be some standard JSON formats but so far they haven’t been located.

These independent receiver channels are apparently a web server of some sort, something that operates using http protocol. Since Rust was already being used, a number of server libraries could be used, such as Actix-web, Rocket, and more. The code for this CSP Endpoint is written and compiled using Rust and Actix-web. 

The lack of a standard reporting JSON format complicated the design of the CSP Reporting Endpoint. How can a Rust struct be defined and compiled if the components of the struct are not known at compile time? The current decision is that all messages will be received as plain text instead of JSON and once the reporting format becomes apparent, the messages will be received as JSON so the data fields of interest can be more easily located.

Rust, as a compiled language, has the additional advantage that there will be no interpretation or evaluation of the received messages. This means that if a bad actor attempts to hijack the CSP report server using malicious js code or python code, it won’t be interpreted. It will simply receive it.

The server won’t try to load the received messages to a database, at least not initially. Instead it will save the messages to a plain text file. This should prevent any threats of injected sql code from being processed. Two sources of potential issues: buffer overruns and multi-byte UTF-8 code potentially containing malicious code.

The CSP report server will also not have any login or authorization code. Instead the server will only accept post messages. This severely limits the attack surface. There is no user interface to confound, no scripts to hijack and misdirect, no forms to flummox. One can post plain text to a path of your choosing and that’s it.

Currently, the assumption is that the person who deploys the CSP report server will have the necessary rights to access the server and will be able to read the violations log in an SSH session. While the violation reports are probably low-value information to a hacker, it still might provide some information about what can’t be used to hack the site. And that violation information might allow a bad actor to limit his hacking attacks to certain pathways that aren’t necessarily well protected.

The assumption is that the server will have a https address secured by an SSL cert, so that should prevent a bad actor from easily intercepting the violation reports. Also, not using ‘get’ requests with the data on the address line also makes it harder for a hacker.

The code will have a JSON post capability commented out initially. Once the JSON format is identified as standardized, the user can make the appropriate fields available in a struct and then make that location available as well.

Currently, the thought is to make the path to report violations non-logical. One would expect the path to post the violations to be named something like, “err” or “violations” or “rpt”. Instead each user is expected to provide some random string of characters, like l4NAP4 or p8WCWt, or even better, boc6fB/iV2ZMe/s8b3QV/g6Sdd8. In effect, the path behaves like a password. Although this path would be behind and https connection, one has to assume that the elite hacker will find a way to get the path if they really want it.

If one really wants to make the post location harder to find, using a non-std port number would be useful. Perhaps something like port 23719  or 18906. Just be sure to open that port to incoming traffic in your firewall.

When this is released, it will probably also be released as a docker package. If the server is running inside a docker container with only one port available to the outside world, it will be even harder to locate and corrupt. And if the OS is something like Alpine with a greatly reduced attack surface, it will be even harder to corrupt. The secure version of Alpine has all of the normal dev tools removed so a hacker who did manage to gain access to the docker container wouldn’t be able to install new software to help him corrupt the system.


As I learn more about CSP security and the http dangers that lurk in the wilds, this document will be updated.
