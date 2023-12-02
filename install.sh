
# Compile for raspberry pi using cross
cross build  --target=armv7-unknown-linux-gnueabihf

bin=target/armv7-unknown-linux-gnueabihf/debug/gc9a01-lcd-firmware

# Send the executable to raspberry pi
echo "Sending executable to remote"
scp $bin "pi@pi:"
echo "Sent ..."