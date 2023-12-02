
# Compile for raspberry pi using cross
cross build  --target=armv7-unknown-linux-gnueabihf

bin=target/armv7-unknown-linux-gnueabihf/debug/gc9a01-lcd-firmware
pi_host="192.168.12.42"

# Send the executable to raspberry pi
echo "Sending executable to remote"
scp $bin "$pi_host:"
echo "Sent ..."