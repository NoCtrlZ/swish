#!/bin/sh

cd test
docker-compose build
docker-compose up &
Status=`docker-compose ps client | awk 'NR==3 { print $5 }'`
Counter=0
while [ "$Status" != '0' ]
do
    echo `docker-compose ps | awk 'NR==5 { print $1 }'`
    Status=`docker-compose ps client | awk 'NR==3 { print $5 }'`
    Counter=$(($Counter + 1))
    if [ $Counter -ge 10 ] || [ "$Status" == '1' ]; then
        echo "Jest Test Failure"
        exit 1
    fi
    sleep 2
done
echo "Jest Test Success"
docker-compose down
