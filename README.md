# jb-name-proxy
proxy to change the sent username by jetbrains programs so that you can get free access to license servers (for educational reasons of course)

## compiling
a simple `cargo build --release` should do the trick in compiling,
there is a chance that you'll need the openssl headers installed to compile but i have not checked

## usage
set the environment variable `JB_USER` to the username you'd like to replace your own with,
and run the program! a folder should be created called `ca` with the files `ca/cert` and `ca/key`  
**DO NOT SHARE THESE FILES**  
if you don't want jetbrains to complain every minute or so that there are untrusted certificates,
simply copy the `ca/cert` file to somewhere under `/etc/ssl/certs/` on linux
(or look at https://www.jetbrains.com/help/idea/ssl-certificates.html) for information on
other operating systems  
  
you should now be able to set your jetbrains ide's proxy to `HTTP` with hostname `127.0.0.1`
and port `6969` (see [configuration](#configuration) for more info)

## configuration
you should be able to set environment variables `JB_HOST` and `JB_PORT` to change what hostname
and port the program binds to