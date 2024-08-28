#!/bin/sh
CUR_HOST=hostname
ip addr
ls -lh /app/
if [ X"$CUR_HOST" == X"node01" ];then
	ip addr
	$(sleep 3;wget http://node02:8080) &
	
else
	$(sleep 3;wget http://node01:8080) &
	
fi

/app/parrot-node --webui-ip=0.0.0.0 --verbose --config-file=/app/parrot-node.toml