alle target löschen:
find . -type d -name target -exec rm -rf {} +

Zeilenendungen löschen:
sed -i 's/\r$//' starten.bash

ausführen:
sudo -E env PATH="/home/stefan/.cargo/bin:$PATH" bash starten.bash

