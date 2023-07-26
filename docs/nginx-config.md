``` \etc\nginx\sites-available\default
server {
        listen 80 default_server;
        listen [::]:80 default_server;
        return 301 https://$host$request_uri;
}

server {
       listen       443 ssl http2 default_server;
       listen       [::]:443 ssl http2 default_server;
       server_name  www.unorthodoxdev.net;
       root         /var/www/unorthodoxdev.net/public;
       index index.html index.htm index.nginx-debian.html;

       ssl_certificate "/etc/letsencrypt/live/www.unorthodoxdev.net/fullchain.pem";
       ssl_certificate_key "/etc/letsencrypt/live/www.unorthodoxdev.net/privkey.pem";
       ssl_session_cache shared:SSL:1m;
       ssl_session_timeout  10m;

       location / {
              try_files $uri $uri/ =404;
       }

       location /comment/ {
	      proxy_pass http://0.0.0.0:8080/;
          proxy_set_header X-Real-IP $remote_addr;
          proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for; 
       }
}
```

## Making Changes

Remember, you must be sudo.
You must restart. (`sudo systemctl restart nginx`)

It also helps to make sure the app is running, and is actually running on port 8080 (`cargo run --release`);