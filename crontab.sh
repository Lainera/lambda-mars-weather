#!/bin/sh

printenv | grep MONGODB_URI >> /etc/environment

cat<<EOF > /etc/cron.d/crontab
MAILTO=""
*/10 * * * * /bootstrap >> /var/log/lambda.log 2>&1

EOF

chmod 0644 /etc/cron.d/crontab
crontab /etc/cron.d/crontab

cron -f
