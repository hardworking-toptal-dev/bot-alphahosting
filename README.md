# ACME Client Automating DNS-01 Challenge for Alfahosting Domains

This is an ACME (letsencrypt by default) client that performs DNS-01 proofs agains an alfahosting standard contract with browser DNS settings.

If you like my work consider supporting me over at [Patreon](https://www.patreon.com/lukaswagner).

## Config file

There needs to be a file `/etc/certbot-alfahosting/config.toml` containing the following entries:

```toml
# certpath = "/etc/letsencrypt"

[alfahosting]
username = "alfauser"
password = "alfapassword"
## id of the contract (usually called "NameX" where X can be a number such as 30)
ipid = "000000"

# this is where you configure the email address the alfahosting login code is sent to.
[imap]
domain = "imap.example.com"
port = 993
username = "user@example.com"
password = "password"

[acme]
# directory_url = "https://acme-v02.api.letsencrypt.org/directory"
account = "user@example.com"

[domains]
## IDs on the right hand side represent the ID of the domain in the
## Alfahosting DNS configuration. It can be acquired by inspecting the
## DOM on that page.
"*.example.com" = "000000"
"*.another-domain.com" = "123456"
```

Note, that it needs the email address in order to check for the code that is sent to your email address whenever you try to log in to your alfahosting account from a new browser.

## The docker container

In order for the client to work best and in order to play nice with your ACME provider this container is best run as a weekly cron job.

```crontab
0 0 * * 0 docker run --rm -v /etc/letsencrypt:/etc/letsencrypt -v /etc/certbot-alfahosting:/etc/certbot-alfahosting certbot-alfahosting:latest
```

Please feel free to randomize the numbers set to zero in this example as to not hit the letsencrypt servers with too high of a load at that time.


