#!/bin/sh
set -e

# Write ACL file to /data (owned by UID 999), not /etc/redis
cat >/data/users.acl <<EOF
user default off
user app on +@read +@write +@list +@set +@hash +@stream +@geo +@hyperloglog +@bitmap -@dangerous ~* >${REDIS_APP_PASSWORD}
user admin on +@all ~* &* >${REDIS_ADMIN_PASSWORD}
EOF

chmod 600 /data/users.acl

# Start Redis. Either use redis.conf (with aclfile directive inside)
# or pass --aclfile here, not both.
exec redis-server /usr/local/etc/redis/redis.conf --aclfile /data/users.acl
