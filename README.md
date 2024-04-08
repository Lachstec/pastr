# Pastr - Self hostable pastebin

## About
Pastr is a simple and self hostable pastebin for sharing source, textfiles and other things written in Rust using actix web.

## Setup
You can use the docker image to get started. The only thing required is a configuration file that 
contains various information needed by pastr. The format of the config file is as follows:
```yaml
# Basic app configuration
app:
  # Port to serve on. You should not serve pastr directly, use a reverse proxy like nginx
  port: 8080
  # Pepper value that gets included with password hashes for increased security.
  pepper: "test_pepper_dont_use"
  # Base URL for pastr. Used to construct links in responses and emails.
  base_url: "test_url"
  # Pastr uses Sendgrid to send E-Mails for confirming new registrations.
  # The Api Key goes here.
  sendgrid_key: "your_sendgrid_api_key"
# Database settings - used to store users and pastes.
database:
  # Hostname of the Database Server to use
  host: "localhost"
  # Name of the database.
  database: "pastr"
  # Username to use for connections
  username: "postgres"
  # Password to use for connections
  password: "test12345"
  # Port of the postgres database
  port: 5432
  # Wether to use TLS for the connection or not
  use_tls: false
```
