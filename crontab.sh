#!/bin/sh

printenv | grep MONGO_URI >> /etc/environment

cat<<EOF > /etc/cron.d/crontab
MAILTO=""
* */12 * * * /bootstrap >> /var/log/lambda.log 2>&1

EOF

chmod 0644 /etc/cron.d/crontab
crontab /etc/cron.d/crontab

cron -f
