# Exits if one of the command returns exit status other than 0
set -e
# Compile for raspberry pi using cross
cross build  --target=armv7-unknown-linux-gnueabihf

bin=target/armv7-unknown-linux-gnueabihf/debug/gc9a01-lcd-firmware

# Send the executable to raspberry pi
echo "Sending executable to remote"
scp $bin "pi@pi:"
echo "Sent ..."

echo "Running it ..."
ssh pi@pi './gc9a01-lcd-firmware'