ifconfig en0 | grep 'inet ' | awk '{print $2}'


ifconfig -l | xargs -n1 ifconfig  | grep 'inet' | awk '{print $2}'
 
