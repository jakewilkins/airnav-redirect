Airnav Redirect
===============

Essentially I'm taking the form and switching it from a POST to a GET so that
searches can be linked to.


      heroku create --buildpack https://github.com/emk/heroku-buildpack-rust.git


Because I'm just that lazy and I'm waiting on a DuckDuckGo Bang command, I have
nginx config in the nginx I have running locally.

      server {
        listen 80;

        server_name a.n;

        access_log /var/log/nginx/airnav-local.log;
        error_log  /var/log/nginx/airnav-local.log;
        location / {
          return 302 https://your-heroku-app.herokuapp.com.com$request_uri;
        }
      }
